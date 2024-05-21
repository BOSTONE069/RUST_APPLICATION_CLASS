// use std::io;

mod fruits;
use fruits::print_fruits;

mod countries;
use countries::print_ccountries;

mod cities;
use cities::city::cities_world;
fn main() {

    let input_1: &str = "23";

    let input_number: i32 = input_1.parse().expect("Was expecting an integer number");
    println!("{}", input_number);

    cities_world();
    print_ccountries();
    print_fruits();

    let number_1 = 101;

    let mut number_to_string: String = number_1.to_string();

    number_to_string.push_str("people in the room");

    println!("Hey {}", number_to_string);

    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}
