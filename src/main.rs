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

    let number_1 = 101;

    let mut number_to_string: String = number_1.to_string();

    number_to_string.push_str("people in the room");

    println!("Hey {}", number_to_string);

    // Variables
    let x = 5;

    println!("The value of x is: {}", x);

    let x = 6;

    println!("The value of x is: {}", x);

    //Constant variables
    const MAX_POINTS: u32 = 100_000;

    println!(
        "The value of MAX_POINTS is: {}",
        MAX_POINTS
    );

    //shadowing
    let _y = 6;
    let _y = 7;

    println!("The value of y is: {}", _y);

    //floating point datatypes

    let d = 10.22;
    println!("This is value of d: {}", d);

    //addition of numbers

    let sum = 5 + 10;

    println!("The sum is: {}", sum);

    let difference = 20 - 10;

    println!("The difference is: {}", difference);

    let product = 20*30;

    println!("The product is: {}", product);

    let quotient = 92 /2;

    println!("The quotient is: {}", quotient);

    let remainder = 43 % 3;

    println!("The remainder is: {}", remainder);

    //Boolean

    let _t = true;
    let _f = false;

    //charater datatypes
    let _r = 'z';

    //Tupple datatypes and how and how to distruvture them in usage

    let tup :(i32, f64, u8) = (-400, 6.3, 1);

    let (q,w, e) = tup;

    println!("The value of q is: {}", q);
    println!("the value of w is: {}", w);
    println!("The value of e is: {}", e);



    println!("Hello, world!");

    print_fruits();

    // array data types

    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];


    //accessing arrays using the index
    println!(
        "The value of months[0] is: {}",
        _months[0]
    );

    //accessing the arrays using the index
    println!("Tha valuse of _a[4] is: {}", _a[4]);


    //for loop to display the array
    for month in _months.iter() {
        println!("{}", month);
    }

    //calling  a function for getting age
    get_age(23);


    //calling the function for adding two numbers
   let sum = add_two_numbers(3, 5);
   println!("The sum is {}", sum);

   conditionals();

//    input_output();


}


fn get_age(age: i32){
    println!("Your age is {}", age);
}


/// The function `add_two_numbers` in Rust takes two unsigned 64-bit integers as input and returns their
/// sum.
/// 
/// Arguments:
/// 
/// * `x`: The parameter `x` is a 64-bit unsigned integer (u64) representing the first number to be
/// added.
/// * `y`: The parameter `y` in the `add_two_numbers` function represents the second number that will be
/// added to the first number `x`.
/// 
/// Returns:
/// 
/// The function `add_two_numbers` returns the sum of the two input numbers `x` and `y`.
fn add_two_numbers(x: u64, y: u64)->u64{
    return x + y;
}


/// The function `conditionals` checks if a number is equal to 5 and prints a corresponding message.
fn conditionals(){
    let number =3;

    if number == 5 {
        println!("The number is 3")
    } else {
        println!("The number is not 5")
    };
}


//function that takes input from user and output the input 

// fn input_output() {
//     let mut input = String::new();
//     println!("Please enter something: ");
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     println!("You entered: {}", input);
// }
