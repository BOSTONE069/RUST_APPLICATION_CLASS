pub mod helpers;
pub mod closures;
pub mod matches;

fn main() {
    println!("Hello, world!");

    //test_func();

    let results = helpers::namehelpers::get_full_name("world", "kenya");

    println!("Hello from {0}", results);

    let new_age = helpers::privatefns::get_age_plus(17);

    println!("The new age is {0}", new_age);

    //test_if();

    // test_while();

    // test_loop();

    // test_forloop();

    closures::test_closures();

    matches::test_match_int();
    matches::test_match_string();
    matches::test_arrays();

}



//Condtional and looping in rust
#[allow(dead_code)]
fn test_forloop() {
    let ages = [16, 14, 26, 12, 45];

    let age_to_drive = 16i32;

    for value in ages {
        if value >= age_to_drive {
            println!{"You are old enogh to drive"}
        }
        else {
            println!("You need to wait for some time more....")
        }
    }
}

#[allow(dead_code)]
fn test_loop() {
    let mut x = 1;
    loop{
        println!("Hello, world!");
        if x > 5 {
            break;
        }
        x = x + 1;
    }
}

#[allow(dead_code)]
fn test_while(){

    let age_to_drive: u8 = 16u8; 

    let mut current_age: u8 = 0u8;


    while current_age < age_to_drive {
        println!("Waiting....");
        current_age += 1;

        if current_age == 4 {
            break;
        }
    }
}

#[allow(dead_code)]
fn test_if(){
    let age_to_drive: u8 = 17u8;

    println!("Enter the persons age to drive:");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing drivers license, because the person is old enough")
    }
    else if age == 16 || age > 14{
        println!("You are just on the verge of being old engough to tget the driver license")
    }
    else {
        println!("Wait abit longer untill you attain the age of the person")
    }

    let _drivers_license = if age >= 16 {true} else {false};


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