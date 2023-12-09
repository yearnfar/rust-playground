struct User {
    name: String,
    age: i32,
}

trait Human {
    fn say_hello(&self) -> String;
}

impl Human for User {
    fn say_hello(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old",
            self.name, self.age
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let user = User {
            name: String::from("John"),
            age: 25,
        };

        assert_eq!(
            user.say_hello(),
            "Hello, my name is John and I am 25 years old"
        );
    }
}
