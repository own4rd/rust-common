#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

//    enum IpAddr {
    //     V4(String),
    //     V6(String),
    // } -> Could use String::from ...
    //   let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("0.0.0.0"),
    };

    println!("{:?}", home.kind);
    
}
