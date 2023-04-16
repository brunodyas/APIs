use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub users: HashMap<u32, User>,
    pub last_id: u32,
}

impl Database {
    pub fn new() -> Self {
        Database {
            users: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn load(path: &Path) -> Result<Self, Error> {
        let file = File::open(path)?;
        let database = serde_json::from_reader(file)?;
        Ok(database)
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}
