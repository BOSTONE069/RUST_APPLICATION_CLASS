#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


pub fn value_in_cents(coin: Coin) -> u8 {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!(
                "State quarter from {:?}!",
                state
            );  25
    }
    };

    println!("The value of the coin is: {}", value);
    value

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5),
let six = plus_one(five);
let none = plus_one(None)