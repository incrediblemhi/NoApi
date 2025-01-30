use template::User;

pub fn add(email: String, password: String) -> User {
    User { email, password }
}
