use std::collections::HashMap;
use crate::models::{User, NewUser};
use crate::db;

pub fn get_users(db: &db::Database) -> Vec<User> {
    db.lock().unwrap().users.clone()
}

pub fn get_user_by_id(db: &db::Database, id: u32) -> Option<User> {
    let users = &db.lock().unwrap().users;
    users.iter().find(|user| user.id == id).cloned()
}

pub fn create_user(db: &db::Database, new_user: NewUser) -> Result<User, String> {
    let mut database = db.lock().unwrap();
    let next_id = database.users.last().map_or(1, |u| u.id + 1);
    let user = User { id: next_id, name: new_user.name, email: new_user.email };
    if database.users.iter().any(|u| u.email == user.email) {
        Err(String::from("User with the same email already exists"))
    } else {
        database.users.push(user.clone());
        Ok(user)
    }
}

pub fn update_user(db: &db::Database, id: u32, updates: HashMap<String, String>) -> Result<User, String> {
    let mut database = db.lock().unwrap();
    let user = database.users.iter_mut().find(|user| user.id == id).ok_or(String::from("User not found"))?;
    for (key, value) in updates {
        match key.as_str() {
            "name" => user.name = value,
            "email" => {
                if database.users.iter().any(|u| u.email == value) {
                    return Err(String::from("User with the same email already exists"))
                }
                user.email = value;
            }
            _ => return Err(format!("Invalid field: {}", key)),
        }
    }
    Ok(user.clone())
}

pub fn delete_user(db: &db::Database, id: u32) -> Result<(), String> {
    let mut database = db.lock().unwrap();
    let index = database.users.iter().position(|user| user.id == id).ok_or(String::from("User not found"))?;
    database.users.remove(index);
    Ok(())
}
