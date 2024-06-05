//use std::io;

mod fruits;
use fruits::print_fruits;

mod countries;
use countries::print_ccountries;

mod cities;
use cities::city::cities_world;

mod enums;
use enums::enumms;

mod structs;
use structs::structs_datatypes;

mod ownership;
use ownership::ownerships;

mod borrowing;
use borrowing::borrowing;


mod conditionalandloops;
use conditionalandloops::conditions;

mod collections;
use collections::collections;

mod hashmaps;
use hashmaps::hash_maps;

mod vectors;
use vectors::vectors;

mod structs_structure;
use structs_structure::print_area;

mod methods;
use methods::methods;

mod matchflow;
use matchflow::value_in_cents;
pub use matchflow::Coin;
fn main() {

    methods();

    print_area();

    vectors();

    hash_maps();

    collections();

    conditions();

    borrowing();

    ownerships();

    structs_datatypes();

    enumms();

    let input_1: &str = "23";

    let input_number: i32 = input_1.parse().expect("Was expecting an integer number");
    println!("{}", input_number);

    // using as key word for casting in the program
    let input_2 : i64 = input_number as i64;

    println!("{}", input_2);

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

    let mut s = String::from("Hello world");

    change(&mut s);


    let reference_to_nothing = dangle();

    println!("{reference_to_nothing}");

    let word = first_word(&s);

    println!("{}", word);

    s.clear();

    let penny = Coin::Penny;


    value_in_cents(penny);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//mutable references
fn change(some_string: &mut String){
    some_string.push_str(", world");
}

//dangling references
fn dangle() -> String {
    let s = String::from("Hello");

    s
}

//Slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}


// fn second_word(s: &String) -> &str {
//     let bytes = s.bytes();

//     for(i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }