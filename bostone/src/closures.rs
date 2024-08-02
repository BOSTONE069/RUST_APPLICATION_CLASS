struct Person {
    first_name: String,
    last_name: String,
}



pub fn test_closures() {

    let add = |x, y| {
        print!("x: {}, y: {}", x, y); 
        x + y
    };

    let results = add(3, 8);

    let print_result = |x| println!("The result is: {}", (results + x));

    print_result(93);

    
    let mut p1 = Person{first_name: "Bostone".to_string(), last_name: "Ochieng".to_string()};

    let mut change_name = |new_first_name: &str| p1.first_name = new_first_name.to_string(); 

    change_name("Rhemney");

    println!("{}", p1.first_name);


}