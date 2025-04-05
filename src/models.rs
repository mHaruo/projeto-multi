use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub linkedin: String,
    pub github: String,
    pub twitter: String,
    pub stars: u32,
    pub stars_given: u32
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub linkedin: String,
    pub github: String,
    pub twitter: String,
}
