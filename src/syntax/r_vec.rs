fn run() {
    let arr = vec![1, 2, 3];
    println!("{:?}", arr);

    let mut arr2 = Vec::new();
    arr2.push(3);
    arr2.push(2);
    arr2.push(1);
    println!("{:?}", arr2);

    for idx in 0..2 {
        if let Some(&m) = arr2.get(idx) {
            println!("{}", m);
        } else {
            println!("None")
        }
    }

    let v = match arr2.get(1) {
        Some(m) => m,
        None => &0,
    };
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run();
    }
}
