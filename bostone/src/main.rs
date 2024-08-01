pub mod helpers;


fn main() {
    println!("Hello, world!");

    //test_func();

    let results = helpers::namehelpers::get_full_name("world", "kenya");

    println!("Hello from {0}", results);
}



#[allow(dead_code)]
fn test_func(){

    let x: f64 = 255.0;

    let y: u8 = x as u8 - 5;

    println!("{}", y);

    let boss = true;

    println!("{}", boss);

    let mut first_name = "Bostone";

    println!("{}", first_name);

    first_name = "Ochieng";

    println!("{}", first_name);

    let data = ("bostone", "Ochieng", 1,2,4);

    println!("{:?}", data);

    let ages = [30,40,56,78,34,56];

    println!("{:?}", ages);
} 