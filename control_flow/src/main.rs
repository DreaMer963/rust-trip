
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => println!("Lucky Penny: 1"),
        Coin::Nickel => println!("Lucky Nickel: 5"),
        Coin::Dime => println!("Lucky Dime: 10"),
        Coin::Quarter => println!("Lucky Quarter: 20"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    value_in_cents(penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five.unwrap(), six.unwrap(), none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let:只匹配一个模式时使用

}