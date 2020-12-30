use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub age: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn user_has_username() {
        let user = User {
            email: String::from("John.Doe@example.com"),
            username: String::from("John"),
            age: 110,
        };
        assert_eq!(user.email, "John.Doe@example.com");
        assert_eq!(user.username, "John");
        assert_eq!(user.age, 110);
    }
}
