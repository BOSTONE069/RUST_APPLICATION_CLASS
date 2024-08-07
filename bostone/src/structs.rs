struct Person {
    first_name: String,
    last_name: String,
    birth_year: u32,
    birth_month: u8,
}

fn new_person() -> Person {
    let p1 = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        birth_month: 6,
        birth_year: 1986,
    };
    return p1;
}

pub fn test_create_person() {
    let myperson: Person = new_person();
    println!("First name: {}, Last name: {}, birth month: {}, birth year: {}", myperson.first_name, myperson.last_name, myperson.birth_month, myperson.birth_year);
}