use std::collections::HashMap;


pub fn hashmaps() {
    let mut map = HashMap::new();

    map.insert(1, 2);

    println!("{:?}", map);
}