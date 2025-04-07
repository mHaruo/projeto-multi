use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;


#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub linkedin: String,
    pub github: String,
    pub twitter: String,
    pub stars: u32,
    pub given_today: u32,
    pub last_given_today: Option<NaiveDate>,
    pub badge: String
}


#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub linkedin: String,
    pub github: String,
    pub twitter: String,
}
