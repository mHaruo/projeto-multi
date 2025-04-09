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
    pub badge: String,
    pub skills: Option<Vec<String>>,
    pub extra_links: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub linkedin: String,
    pub github: String,
    pub twitter: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub twitter: Option<String>,
    pub skills: Option<Vec<String>>,
    pub extra_links: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub votes: Vec<Vote>,
}
#[derive(Deserialize)]
pub struct NewProject {
    pub title: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VoteOption {
    Sim,
    Nao,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Vote {
    pub user_id: Uuid,
    pub weight: u32,
    pub in_favor: VoteOption,
}

