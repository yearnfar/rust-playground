fn run() {
    let x = 5;
    let y = Box::new(5);

    let z = x + *y;

    println!("z = {}", z);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn it_works() {
        run()
    }
}
