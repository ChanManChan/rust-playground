#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_counter: u64,
    active: bool
}

impl User {
    fn increment_signin_count(&mut self) {
        self.sign_in_counter += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let mut user_1 = User {
            username: String::from("Nandu"),
            email: String::from("nandu@test.com"),
            active: true,
            sign_in_counter: 1
        };

        change_username(&mut user_1, "Nanda Gopal");
        
        dbg!(user_1);

        let mut user_2 = User {
            username: String::from("Nandu2"),
            email: String::from("nandu2@test.com"),
            active: false,
            sign_in_counter: 5
        };

        user_2.increment_signin_count();
        user_2.change_email("newemail@test.com");

        dbg!(user_2);
    }
}