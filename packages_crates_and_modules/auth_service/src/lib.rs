#![allow(dead_code, unused_variables)]

// It makes the code easy to organize using modules
pub mod auth_utils;
mod database;

use auth_utils::{login, models::Credentials};
use database::{connect_to_database, DbStatus};

pub fn authenticate(cred: Credentials) {
    if let DbStatus::Connected = connect_to_database() {
        login(cred);
    }
}
