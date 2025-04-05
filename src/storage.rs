use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::models::User;
use uuid::Uuid;

lazy_static! {
    pub static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

pub fn add_user(user: User) {
    USERS.lock().unwrap().push(user);
}

pub fn get_users() -> Vec<User> {
    USERS.lock().unwrap().clone()
}

pub fn give_star(id: Uuid) -> bool {
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.stars += 1;
        true
    } else {
        false
    }
}
