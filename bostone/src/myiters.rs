pub fn test_rust_iterator(){
    let fruit_list = vec!["Strawberry", "Avocado", "Carrot", "Banans", "Apple"];

    let nut_list = vec!["Walnut", "Peanuts", "Almonds", "Groundnuts"];

    

    let mut fruit_iter = fruit_list.iter();

    // for fruit in fruit_iter {
    //     println!("{}", fruit);
    // }

    let item01 = fruit_iter.next().unwrap();

    println!("{}", item01);

    let chian_list = fruit_list.iter().chain(nut_list.iter());

    let all_foods: Vec<&&str> = chian_list.clone().collect();

    for food in chian_list {
        println!("{}", food);
    }

    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));

    let new_fruits = fruit_list_strings.map(|mut e: String| {e.push_str(" fruit"); return e});


    new_fruits.for_each(|e| println!("{}", e));



}