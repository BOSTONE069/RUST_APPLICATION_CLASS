pub fn test_rust_iterator(){
    let fruit_list = vec!["Strawberry", "Avocado", "Carrot", "Bananas", "Apple"];

    let nut_list = vec!["Walnut", "Peanuts", "Almonds", "Groundnuts"];

    

    let mut fruit_iter = fruit_list.iter();

    // for fruit in fruit_iter {
    //     println!("{}", fruit);
    // }

    let item01 = fruit_iter.next().unwrap();

    println!("{}", item01);

    // `let chian_list = fruit_list.iter().chain(nut_list.iter());` is creating a new iterator called
    // `chian_list` that chains together the iterators of `fruit_list` and `nut_list`. This means that
    // the elements of `nut_list` will be iterated over after the elements of `fruit_list`, effectively
    // combining the two lists into a single iterator.
    let chian_list = fruit_list.iter().chain(nut_list.iter());

    let all_foods: Vec<&&str> = chian_list.clone().collect();

    for food in chian_list {
        println!("{}", food);
    }

    // The line `let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));` is creating a
    // new iterator called `fruit_list_strings` by mapping each element in the `fruit_list` vector to a
    // new `String` object.
    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));

    let new_fruits = fruit_list_strings.map(|mut e: String| {e.push_str(" fruit"); return e}); //allows addition of fruit strings


    new_fruits.clone().for_each(|e| println!("Final names of the fruits include: {}", e)); //useing closures for the renaming

    println!("Last fruit in the list is: {}", new_fruits.clone().last().unwrap());// getting the last element in the list

    let mut stepby = new_fruits.clone().step_by(2); //Alternating step in the code list

    println!("Step: {}", stepby.next().unwrap());
    println!("Step: {}", stepby.next().unwrap());
    println!("Step: {}", stepby.next().unwrap());

    let first_names = vec!["Bostone", "Rhemney", "Joyce", "Stephanie"];
    let first_name_strings = first_names.iter().map(|e| String::from(*e));

    let last_names = vec!["Ochieng", "Okeke", "Adhiambo", "Mutua"];
    let last_name_strings = last_names.iter().map(|e| String::from(*e));


    let full_names = first_name_strings.zip(last_name_strings);

    full_names.for_each(|name| println!("{} {}", name.0, name.1));

}