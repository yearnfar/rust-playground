use std::fmt::Display;

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn run() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home, : {}", home.address);
    println!("loopback, address: {}", home.address);
}

fn adder(a: i32, b: i32) -> Option<i32> {
    if a + b < 100 {
        Some(a + b)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        run()
    }

    #[test]
    fn adder_test() {
        if let Some(n) = adder(100, 10) {
            println!("{}", n)
        } else {
            println!("none")
        }
    }
}
