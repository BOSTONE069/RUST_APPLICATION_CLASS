pub fn test_vec_int() {
    let mut my_int: Vec<i32> = Vec::new();

    my_int.push(30);
    my_int.push(40);
    my_int.push(50);
    my_int.push(60);
    my_int.push(70);
    my_int.push(80);


    my_int.len();

    println!("Size of vec {:?}", my_int.len());
    println!("Capacity of vec {:?}", my_int.capacity());
    println!("{:?}", my_int);

    println!("First element in the vec is {}", my_int[1]);

    println!("First element in the vec is {:?}", &(&my_int).as_slice()[0..4]);

    println!("First element in the vec is: {:?}", my_int.get(2));

    
}

pub fn test_vec_string() {
    let first_name: Vec<&str> = vec!["Bostone", "Rhemney", "Velma", "James", "Milton"];

    for first_name in first_name {
        println!("{}", first_name);
    }
}