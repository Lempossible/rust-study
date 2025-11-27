use std::net::{Ipv4Addr, Ipv6Addr};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call");
    }
}

fn main() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    println!("Hello, world! {}", ip);

    let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    println!("Hello, world! {}", ipv6);

    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Write(s) => println!("{}", s),
        _ => {msg.call();}
    }
}

