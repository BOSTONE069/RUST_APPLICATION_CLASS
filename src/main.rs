mod fruits;
use fruits::print_fruits;

mod countries;
use countries::print_ccountries;

mod cities;
use cities::city::cities_world;
fn main() {

    cities_world();
    print_ccountries();
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

    //Tupple datatypes

    let tup :(i32, f64, u8) = (-400, 6.3, 1);

    let (q,w, e) = tup;

    println!("The value of q is: {}", q);
    println!("the value of w is: {}", w);
    println!("The value of e is: {}", e);



    println!("Hello, world!");

    print_fruits();
}
