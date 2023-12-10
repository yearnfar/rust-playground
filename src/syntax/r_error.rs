fn run_panic() {
    panic!("test panic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_panic() {
        run_panic();
    }
}
