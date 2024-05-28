#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



pub fn print_area(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The rectangle is {:#?} square pixels.", rect1);
}
