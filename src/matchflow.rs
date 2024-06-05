enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


pub fn value_in_cents(coin: Coin) -> u8 {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    println!("The value of the coin is: {}", value);
    value

    
}