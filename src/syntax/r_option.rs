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

    println!("home: {}", home.address);
    println!("loopback: {}", loopback.address);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        run()
    }
}
