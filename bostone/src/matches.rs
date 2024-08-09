pub fn test_arrays(){
    let prices = [50000, 90000, 120000];

    // This code snippet is using pattern matching in Rust to check if the slice of the `prices` array
    // from index 0 to 1 (inclusive) matches the pattern `[30000, 50000]`.
    match prices[0..=1] {
        [30000, 50000] => println!("You have some reseonable price cars"),
        _=> println!("You dont already have a reasonable prices cars")
    }
}



pub fn test_match_string() {
    let car_manufacturer: &str = "Porsche";

    // This code snippet is using pattern matching in Rust to check the value of the `car_manufacturer`
    // variable.
    match car_manufacturer {
        "Hyundia" => {
            println!("Hyundia it is already");
        }

        "Porsche" => {
            println!("Porsche it is already");
        }

        _ => {
            println!("The manufacturere is not in the list");
        }
    }
}

pub fn test_match_int(){
    let myage: u16 = 35;

    // This code snippet is using pattern matching in Rust to check the value of the `myage` variable.
    // The `match` statement is matching the value of `myage` against different patterns.
    match myage {
        1..= 35 => { //Range pattern matching
            println!("Your age is 35");
        }
        _ => {
            println!("You age is not 35 ");
        }
    }
}