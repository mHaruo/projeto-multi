use crate::models::{User, UpdateUser};
use lazy_static::lazy_static;
use std::sync::Mutex;
use uuid::Uuid;

lazy_static! {
    pub static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}
pub fn get_users() -> Vec<User> {
    USERS.lock().unwrap().clone()
}

pub fn add_user(user: User) {
    USERS.lock().unwrap().push(user);
}
fn calculate_badge(stars: u32) -> String {
    if stars >= 5 {
        "ouro".to_string()
    } else if stars >= 3 {
        "prata".to_string()
    } else if stars >= 1 {
        "bronze".to_string()
    } else {
        "nenhuma".to_string()
    }
}


pub fn give_star(to_id: Uuid, from_id: Uuid) -> Option<f64> {
    let mut users = USERS.lock().unwrap();

    let from_index = users.iter().position(|u| u.id == from_id)?;
    let to_index = users.iter().position(|u| u.id == to_id)?;
    
    let (first, second) = if from_index < to_index {
        let (left, right) = users.split_at_mut(to_index);
        (&mut left[from_index], &mut right[0])
    } else if from_index > to_index {
        let (left, right) = users.split_at_mut(from_index);
        (&mut right[0], &mut left[to_index])
    } else {        
        return None;
    };

    let today = chrono::Local::now().date_naive();

    if let Some(last_date) = first.last_given_today {
        if last_date == today {
            if first.given_today >= 5 {
                return None;
            } else {
                first.given_today += 1;
            }
        } else {
            first.given_today = 1;
            first.last_given_today = Some(today);
        }
    } else {
        first.given_today = 1;
        first.last_given_today = Some(today);
    }

    second.stars += 1;
    second.badge = calculate_badge(second.stars);


    let cost = 1.0 * 2f64.powi(first.given_today as i32 - 1);
    Some(cost)
}

pub fn update_user(user_id: Uuid, updates: UpdateUser) -> bool {
    let mut users = USERS.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == user_id) {
        if let Some(name) = updates.name {
            user.name = name;
        }
        if let Some(linkedin) = updates.linkedin {
            user.linkedin = linkedin;
        }
        if let Some(github) = updates.github {
            user.github = github;
        }
        if let Some(twitter) = updates.twitter {
            user.twitter = twitter;
        }
        if let Some(skills) = updates.skills {
            user.skills = Some(skills);
        }
        if let Some(extra_links) = updates.extra_links {
            user.extra_links = Some(extra_links);
        }
        return true;
    }
    false
}




