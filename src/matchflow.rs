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