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

    making_entry_into_hashmaps();
}

// create a structs with vectors and hashmaps
fn making_entry_into_hashmaps(){
    let mut map: HashMap<&str, i32> = HashMap::new();

    // Making entries into the HashMap
    map.insert("key1", 10);
    map.insert("key2", 20);

    // Updating values in the HashMap
    if let Some(value) = map.get_mut("key1") {
        *value = 30;
    }

    // Printing the HashMap
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
//call function of the struct to update the vectors and hashmap