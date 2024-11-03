pub fn login(cred: models::Credentials) {
    // try to login the user
    println!("Trying to login the user");
    crate::database::get_user()
}

pub mod models;
