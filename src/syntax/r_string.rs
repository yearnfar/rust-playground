fn run() {
    let s0 = "hello".to_string();
    let s1 = String::from("world");
    let s2: String = "!".into();

    println!("{} {} {}", s0, s1, s2);

    let sparkle_heart = vec![240, 159, 146, 150];

    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{}", sparkle_heart);

    let bytes = sparkle_heart.into_bytes();
    println!("{:#?}", bytes);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        run()
    }
}
