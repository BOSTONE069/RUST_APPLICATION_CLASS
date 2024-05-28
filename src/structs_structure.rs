pub fn print_area(){
    let width1 = 50;
    let height1 = 30;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}



fn area(width:u32, height:u32) -> u32 {
    width * height
}