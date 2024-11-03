pub enum DbStatus {
    Connected,
    Interrupted,
}

pub fn connect_to_database() -> DbStatus {
    // Connect to db logic...
    println!("Connected to DB");
    DbStatus::Connected
}

pub fn get_user() {
    println!("Need to get the user");
    // it should return the user
}
