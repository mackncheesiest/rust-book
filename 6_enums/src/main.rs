#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting!"),
            Message::Move{x, y} => println!("Moving to (x, y): ({}, {})", x, y),
            Message::Write(write_str) => println!("Writing: {}", write_str),
            Message::ChangeColor(r, g, b) => println!("Changing color to (r, g, b): ({}, {}, {})", r, g, b)
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}, loopback: {:?}", home, loopback);

    let mesg_q = Message::Quit;
    let mesg_m = Message::Move{ x: 1, y: 2 };
    let mesg_w = Message::Write(String::from("Some text"));
    let mesg_c = Message::ChangeColor(127, 128, 129);

    mesg_q.call();
    mesg_m.call();
    mesg_w.call();
    mesg_c.call();

    let penny = Coin::Penny;
    println!("The value of a penny is {} cent(s)", penny.value_in_cents());

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("Three!"),
        _ => ()
    }
    //Equivalently:
    if let Some(3) = some_u8_value {
        println!("Three again!");
    }
}
