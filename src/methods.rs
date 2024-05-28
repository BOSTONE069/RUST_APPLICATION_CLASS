#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }
}

pub fn methods(){
    let rect1 = Rectangle {
        width: 50,
        height: 70,
    };

    println!("The area of the reactagmle is {} square pixels.", rect1.area());

    if rect1.width(){
        println!("The recatangle has nonzero width; is is {}", rect1.width);
    }
}