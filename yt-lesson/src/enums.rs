use std::net::IpAddr;

enum IpAddrKind {
    v4,
    v6,
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::v4 => println!("4"),
        IpAddrKind::v6 => println!("6"),
    }
}

fn main() {
    // enums
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(four);
    route(six);

    struct IpAddr {
        kind: IpAddrKind,
        ip_address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::v4,
        ip_address: "127.0.0.1".to_string(),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        ip_address: "::1".to_string(),
    };

    println!("home ip {}", home.ip_address);

    // using enums, recommended
    enum IpAddrEnum {
        v4(String),
        v6(String),
    }
    enum IpAddrEnum2 {
        v4(u8, u8, u8, u8),
        v6(String),
    }
    let homeEnum: IpAddrEnum = IpAddrEnum::v4("127.0.0.1".to_string());
    let loopEnum: IpAddrEnum = IpAddrEnum::v6("::1".to_string());
    let ex2Enum: IpAddrEnum2 = IpAddrEnum2::v4(22, 22, 22, 22);
}

