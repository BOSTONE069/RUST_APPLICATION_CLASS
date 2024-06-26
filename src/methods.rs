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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Associated Functions
    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn methods(){
    //Working with associated functions
    let sq = Rectangle::square(3);
    println!("{:#?}", sq);

    let rect1 = Rectangle {
        width: 50,
        height: 70,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can react1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can react1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area of the reactagmle is {} square pixels.", rect1.area());

    if rect1.width(){
        println!("The recatangle has nonzero width; is is {}", rect1.width);
    }
}