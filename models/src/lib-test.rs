
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn user_has_username() {
        let user = User {
            email: String::from("another@example.com"),
            username: String::from("Joe"),
            age: 110,
        };
        assert_eq!(user.email, "another@example.com");
        assert_eq!(user.username, "anotherusername567");
        assert_eq!(user.age, 1);
    }
}
