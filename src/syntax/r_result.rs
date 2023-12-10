fn adder(a: i32, b: i32) -> Result<i32, i32> {
    if a < 10000 && b < 1000 {
        return Ok(a + b);
    }
    Err(-1)
}

fn adder_no_return(a: i32, b: i32) -> Result<i32, ()> {
    if a < 10000 && b < 10000 {
        return Ok(a + b);
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_works() {
        if let Ok(n) = adder(100, 10000) {
            assert_eq!(n, 10100)
        }
    }

    #[test]
    fn adder_not_works() {
        if let Err(()) = adder_no_return(100000, 10000) {
            println!("not works")
        }
    }
}
