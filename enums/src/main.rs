use std::net::IpAddr;

enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        //method body here
    }
}

fn main() {

    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;


    //FUNCTION THAT TAKES IP ENUM
    // fn route(ip_type: IpAddrKind) {
    //
    // }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    //the below examples are using enums in a struct
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address: String::from("::1"),
    };

    //let's just use enums
    //this is much simpler and just uses enums as opposed to enums and structs together

    enum IpAddrTwo{
        v4(String),
        v6(String),
    }

    let home2 = IpAddrTwo::v4(String::from("127.0.0.1"));
    let loopback2 = IpAddrTwo::v6(String::from("::1"));


    //notice how we can store a v4 in this other way below

    enum IpAddrThree{
        v4(u8, u8, u8, u8),
        v6(String),
    }

    let home3 = IpAddrThree::v4(127, 0, 0, 1);
    let loopback3 = IpAddrThree::v6(String::from("::1"));



    let m = Message::Write(String::from("hello"));
    m.call()

}
