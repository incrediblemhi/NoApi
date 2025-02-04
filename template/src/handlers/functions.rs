#[derive(serde::Serialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
}

pub fn create_user(email: String, password: String, _username: String) -> User {
    User { email, password }
}
