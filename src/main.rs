fn main() {
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
    let y = 6;
    let y = 7;

    println!("The value of y is: {}", y);

    //floating point datatypes

    let d = 10.22;
    println!("This is value of d: {}", d);


    println!("Hello, world!");
}
