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

trait Animal {
    fn make_sound(&self);
}
struct Cat {}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("The cat says 'Meow!'");
    }
}

struct Dog {}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("The dog says 'Woof!'");
    }
}

fn run() {
    let dog = Dog {};
    let cat = Cat {};

    dog.make_sound();
    cat.make_sound();
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

    #[test]
    fn works_run() {
        run()
    }
}
