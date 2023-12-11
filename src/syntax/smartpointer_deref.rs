use std::ops::Deref;

fn run() {
    let x = 5;
    let y = MyBox(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_works() {
        run()
    }
}
