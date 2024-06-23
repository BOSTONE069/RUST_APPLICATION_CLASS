#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}
/// The `Coin` enum in Rust defines different types of coins. It has variants such as `Penny`, `Nickel`,
/// `Dime`, and `Quarter(UsState)`.
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


/// The function `value_in_cents` takes a `Coin` enum variant as input and returns the corresponding
/// value in cents.
///
/// Arguments:
///
/// * `coin`: Coin
///
/// Returns:
///
/// The function `value_in_cents` returns an unsigned 8-bit integer (`u8`) representing the value of the
/// coin passed as an argument.
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

/// The function `plus_one` takes an `Option<i32>` as input and returns an `Option<i32>` with the value
/// incremented by 1 if it exists.
///
/// Arguments:
///
/// * `x`: Option<i32>
///
/// Returns:
///
/// The function `plus_one` takes an `Option<i32>` as input and returns an `Option<i32>`. If the input
/// is `None`, it returns `None`. If the input is `Some(i)`, it adds 1 to `i` and returns `Some(i + 1)`.
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}