use std::collections::HashMap;

pub fn test_hashmap_basic() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();

    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());

    stock_list.insert("NVDA".to_string(), 234.53);
    stock_list.insert("Microsoft".to_string(), 345.90);
    stock_list.insert("TCM".to_string(), 300.90);

    stock_list.insert("NVDA".to_string(), 200.53);

    stock_list.entry("META".to_string()).or_insert(500.56);
    stock_list.entry("KCB".to_string()).or_insert(700.56);


    
    println!("{:#?}", stock_list);

    stock_list.remove(&("Microsoft".to_string()));

    println!("{:#?}", stock_list);

    for (ticker, current_value) in stock_list {
        println!("{} is trading at {}", ticker, current_value);
    }
}