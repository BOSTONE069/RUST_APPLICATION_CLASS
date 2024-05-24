use std::collections::HashMap;

pub fn hash_maps(){
    //hashmaps > key and value

    let mut maps: HashMap<String, i32> = HashMap::new();

    maps.insert("Hello".to_string(), 22);
    maps.insert("World".to_string(), 22);
    maps.insert("Kenya".to_string(), 22);


    println!("{:?}", maps);


    maps.insert("Hello".to_string(), 42);


    println!("{:?}", maps);

    for (key, value) in maps {
        println!("Key {} and Vlaue {}", key, value)
    }
}