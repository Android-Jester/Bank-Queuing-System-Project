use std::time::Duration;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Teller {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerInsert {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: f32,
    pub active: bool,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerLogin {
    pub server_id: String,
    pub password: String,
}

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::data::schema::Tellers)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct TellerQuery {
    pub server_id: String,
    pub server_station: i32,
    pub password: String,
    pub service_time: f32,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum TellerState {
    Active,
    InActive,
    PendingRelease,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]

pub struct TellerQueueQuery {
    pub server_id: String,
    pub server_station: i32,
    pub service_time: Duration,
    pub teller_state: TellerState,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct ServerQueue {
    pub teller: TellerQueueQuery,
    pub users: Vec<UserQueuePos>,
}

impl ServerQueue {
    pub fn new(teller: TellerQueueQuery) -> Self {
        Self {
            teller,
            users: vec![],
        }
    }
}

impl Teller {
    pub fn change_teller_status(&mut self, status: bool) -> &mut Teller {
        self.active = status;
        self
    }
}