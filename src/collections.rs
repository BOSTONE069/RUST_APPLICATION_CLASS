pub fn collections(){

    let x = vec![1,2,3,4];

    println!("{:?}", x);


    for item in &x {
        println!("Looping through {}", item);
    }

    // Using a while loop
    let mut index = 0;

    while index < x.len() {
        println!("Looping through {}", x[index]);
        index += 1;
    }

    vector();

    date_time();




}

fn vector(){
    let mut x = Vec::new();

    x.push(1);
    x.push(2);
    x.push(3);

    for items in &x{
        println!("Looping through the vector {}", items);
    }

    let mut index_vector = 0;

    while index_vector < x.len(){
        println!("While looping through the vector{}", x[index_vector]);
        index_vector += 1;
    }

    x.push(4);

    println!("Mutated vector: {:?}", x);
}


fn date_time(){

    let date_time_string = "2024-05-23T12:34:56";

     // Find the index of 'T' in the string
    let index = match date_time_string.find('T') {
        Some(i) => i + 1,  // Start from the character after 'T'
        None => 0,         // Default to 0 if 'T' is not found
    };

    // Print the values after 'T' in the variable
    if index < date_time_string.len() {
        let values_after_t = &date_time_string[index..];
        println!("Values after 'T' in the variable: {}", values_after_t);
    } else {
        println!("No values found after 'T' in the variable.");
    }
}