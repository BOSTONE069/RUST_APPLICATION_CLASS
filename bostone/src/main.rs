fn main() {
    println!("Hello, world!");

    test_func();
}


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

    
} 