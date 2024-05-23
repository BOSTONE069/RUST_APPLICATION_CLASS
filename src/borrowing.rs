/// The function `borrowing` creates a string `s1`, inserts it into a database, and then prints the
/// value of `s2`.
pub fn borrowing(){
    let s1 = "hello".to_string();

    let s2 = insert_to_db(&s1);

    println!("The value of s2 is: {}", s2);  
}

/// The function `insert_to_db` in Rust takes a reference to a `String` as a parameter and returns a
/// reference to the same `String`.
/// 
/// Arguments:
/// 
/// * `par`: The function `insert_to_db` takes a reference to a `String` as a parameter named `par`. It
/// then returns a reference to the same `String` that was passed as the parameter.
/// 
/// Returns:
/// 
/// The function `insert_to_db` is returning a reference to a `String` type.
fn insert_to_db(par: &String) -> &String {
    par
}