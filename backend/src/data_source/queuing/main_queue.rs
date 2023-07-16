use crate::prelude::*;


#[derive(Debug)]
pub struct MainQueue {
    pub queue: Vec<UserQueuePos>,
}

impl Default for MainQueue {
    fn default() -> Self {
        MainQueue {
            queue: Vec::with_capacity(CUSTOMER_COUNT),
        }
    }
}

impl MainQueue {
    /*Main Queue Events*/

    // adding user
    // 1. Get user's details
    // 2. Assign them to available teller
    // 3. Give them the time
    // 4. Include them into the queue
    // 5. Include them to sub queues

    pub fn add_user(
        &mut self,
        mut added_user: UserQueuePos,
        sub_queue: &mut SubQueues,
    ) -> Result<UserQueuePos, &'static str> {
        match self.queue.iter().find(|user| user.national_id == added_user.national_id) {
            None => {
                let user_position = self.queue.len() + 1;
                if user_position < CUSTOMER_COUNT && sub_queue.teller_count() > 0  {
                    added_user.setup_main(user_position);
                    sub_queue.customer_add(&mut added_user).unwrap();
                    self.queue.push(added_user.clone());
                    Ok(added_user)
                } else if sub_queue.teller_count() == 0 {
                    Err("No Teller Available")
                } else {
                    Err("Queue is Full")
                }
            }
            Some(_) => {
                Err("User already in queue")
            }
        }
    }
    pub fn remove_user<'a>(
        &'a mut self,
        user_queue: UserQueuePos,
        servers: &'a mut SubQueues,
    ) -> Result<UserQueuePos, &'static str> {
        if self.queue.len() > 0 {
            let removed_user = self.queue.remove(user_queue.position - 1);
            self.main_queue_realign(removed_user.position);
            Ok(servers.customer_remove(removed_user.clone()))
        } else {
            Err("No User Available")
        }

    }

    pub fn dismiss_user<'a>(
        &'a mut self,
        national_id: String,
        servers: &'a mut SubQueues,
    ) -> Result<UserQueuePos, &'static str> {
        if self.queue.len() > 0 {
            let user_found = self.queue.iter().find(|user| user.national_id == national_id);
            match user_found {
                None => {
                    Err("No User Available")
                }
                Some(user_data) => {
                    let removed_user = self.queue.remove(user_data.position - 1);
                    self.main_queue_realign(removed_user.position);
                    Ok(servers.customer_remove(removed_user.clone()))
                }
            }
        } else {
            Err("No User Available")
        }

    }
    /*Live Changes*/
    pub fn queue_change(&mut self) {
        for (pos, user) in self.queue.iter_mut().enumerate() {
            user.change_queue_pos(pos + 1);
        }
    }

    fn main_queue_realign(&mut self, old_queue_position: usize) {
        //TODO: Change the sub_queue_position of all users after the removed user

        for (position, user) in self.queue.iter_mut().enumerate() {
            if user.position > old_queue_position {
                user.position = position
            }
        }
    }
}