fn run_panic() {
    panic!("test panic");
}

fn run_result_ok() -> Result<i32, ()> {
    Ok(1)
}
fn run_result_err() -> Result<i32, String> {
    Err(String::from("some thing wrong"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_panic() {
        run_panic();
    }

    #[test]
    fn test_run_result_ok() {
        let ret = match run_result_ok() {
            Ok(n) => n,
            Err(()) => 0,
        };
        println!("ret: {}", ret)
    }

    #[test]
    fn test_run_result_err() {
        let ret = match run_result_err() {
            Ok(n) => n,
            Err(err) => {
                println!("msg: {}", err);
                0
            }
        };
        println!("ret: {}", ret)
    }
}
