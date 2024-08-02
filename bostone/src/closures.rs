pub fn test_closures() {

    let add = |x, y| {
        print!("x: {}, y: {}", x, y); 
        x + y
    };

    let results = add(3, 8);

    let print_result = |x| println!("The result is: {}", (results + x));

    print_result(93);



}