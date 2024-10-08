use crate::prelude::*;

#[derive(Selectable, Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = Guests_Clients)]
#[diesel(check_for_backend(Pg))]
pub struct GuestQuery {
    pub national_id: String,
    pub name: String,
    pub transaction_detail: String,
    pub telephone_num: String,
}
