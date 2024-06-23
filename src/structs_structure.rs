#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


/// The function `print_area` calculates and prints the area of a rectangle with a width of 30 and
/// height of 50.

pub fn print_area(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The rectangle is {:#?} square pixels.", rect1);
}
