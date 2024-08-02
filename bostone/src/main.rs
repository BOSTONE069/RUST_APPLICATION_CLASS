pub mod helpers;


fn main() {
    println!("Hello, world!");

    //test_func();

    let results = helpers::namehelpers::get_full_name("world", "kenya");

    println!("Hello from {0}", results);

    let new_age = helpers::privatefns::get_age_plus(17);

    println!("The new age is {0}", new_age);

    test_if();

}

fn test_if(){
    let age_to_drive: u8 = 16u8;

    println!("Enter the persons age to drive:");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing drivers license, because the person is old enough")
    }
    else {
        println!("Wait abit longer untill you attain the age of the person")
    }
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