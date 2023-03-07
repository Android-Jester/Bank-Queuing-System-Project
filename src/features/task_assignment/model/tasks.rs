use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Completed,
    Incomplete,
    Pending,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub task_id: String,
    pub task: String,
    pub task_name: String,
    #[serde(with = "ts_seconds_option")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub completed_date: Option<DateTime<Utc>>,
    pub task_status: Status,
    pub task_details: String,
    pub task_report: String,
}