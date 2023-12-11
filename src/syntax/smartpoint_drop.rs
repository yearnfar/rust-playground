fn run() {
    let _c = CustomSmartPoint {
        data: "my staff".to_string(),
    };

    let _d = CustomSmartPoint {
        data: String::from("other staff"),
    };
    println!("CustomSmartPoint Created.")
}

struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("drop CustomSmartPoint With Data: `{}`!", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run();
    }
}
