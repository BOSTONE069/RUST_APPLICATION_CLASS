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

    for first_name in first_name.clone() {
        println!("{}", first_name);
    }

    println!("{:?}", first_name);
}

#[derive(Debug)]
#[derive(Clone)]
struct Car {
    manufacturer: String,
    model: String,
    year: u32,
}

pub fn test_vec_car() {
    let mut car_list: Vec<Car> = vec![];

    for _ in 1..100u8 {
        car_list.push(Car{manufacturer:"Porsche".to_string(), model:"Panamera".to_string(), year: 2024});
    }

    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_list.capacity());
}