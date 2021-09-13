#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(String),
}

//Use struct
// struct IpAddress {
//     kind: IpAddrType,
//     address: String,
// }
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColot(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //Use struct
    // let home = IpAddress {
    //     kind: IpAddrType::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loop_back = IpAddress {
    //     kind: IpAddrType::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddrType::V4(127,0,0,1);
    let loopback = IpAddrType::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback);

    let m = Message::Write(String::from("Hello"));

    m.call();

    let some_num = Some(5);
    let some_str = Some("Hello, World");

    let absent_number: Option<i32> = Option::None;

    let x: i8 = 5;
    let y: Option<i8> = Option::Some(5);

    //match

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);
    
    println!("{:?}", six);

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),                   // 1 | 3 | 5 | 7;
    }
    
    //if let
    let some_u8_value: Option<u8> = Option::Some(0u8);

    if let Option::Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> i32{
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,           //if None check is blank, rust will make error message!
        Option::Some(i) => Option::Some(i + 1)
    }
}