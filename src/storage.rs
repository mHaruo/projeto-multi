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

pub fn give_star(from_user_id: Uuid, to_user_id: Uuid) -> Option<f64> {
    let mut users = USERS.lock().unwrap();

    let from_index = users.iter().position(|u| u.id == from_user_id)?;
    let to_index = users.iter().position(|u| u.id == to_user_id)?;

    if from_index == to_index {
        return None; // Não pode dar estrela para si mesmo
    }

    // Evita dois &mut simultâneos usando duas chamadas separadas
    let from_ptr: *mut User = &mut users[from_index];
    let to_ptr: *mut User = &mut users[to_index];

    unsafe {
        (*from_ptr).stars_given += 1;
        (*to_ptr).stars += 1;
    }

    let cost = 1.2_f64.powi(unsafe { (*from_ptr).stars_given as i32 });

    Some(cost)
}


