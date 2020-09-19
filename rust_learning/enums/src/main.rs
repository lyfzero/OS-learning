enum IpAddrKind {
    V6(String),
    V4(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}
fn main() {
    // 将数据附加到枚举的每个成员，替代结构体
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    // option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // match
    let coin = Coin::Penny;
    println!("coin in cents is: {}", value_in_cents(coin));
    let coin1 = Coin::Quarter(UsState::Alaska);
    println!("coin in cents is: {}", value_in_cents(coin1));

    // if let
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quater from {:?}!", state);
    } else {
        count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}