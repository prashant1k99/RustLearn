use auth_service::{auth_utils::models::Credentials, authenticate};

fn main() {
    let creds = Credentials {
        username: String::from("prashant1k99"),
        password: String::from("my_secret_password"),
    };

    authenticate(creds);
}
