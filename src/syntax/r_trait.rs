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
    let ani = rand_animal(1);
    ani.make_sound();

    let ani2 = rand_animal(11);
    ani2.make_sound();
}

fn rand_animal(i: i32) -> Box<dyn Animal> {
    if i < 10 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
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

    #[test]
    fn works_run() {
        run()
    }
}
