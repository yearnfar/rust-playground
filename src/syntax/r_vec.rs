fn run() {
    let arr = vec![1, 2, 3];
    println!("{:?}", arr);

    let mut arr2 = Vec::new();
    arr2.push(3);
    arr2.push(2);
    arr2.push(1);
    println!("{:?}", arr2);
    if let Some(m) = arr2.get(0) {
        println!("{}", m);
    } else {
        println!("None")
    }

    if let Some(m) = arr2.get(199) {
        println!("{}", m);
    } else {
        println!("None")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
