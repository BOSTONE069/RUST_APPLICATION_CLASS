/// The function `borrowing` creates a string `s1`, inserts it into a database, and then prints the
/// value of `s2`.
pub fn borrowing(){
    let mut s1 = "hello".to_string();

    let s2 = insert_to_db(& mut s1);

    println!("The value of s2 is: {}", s2);  
}

/// The function `insert_to_db` in Rust appends "World" to a mutable string reference and returns the
/// modified string reference.
/// 
/// Arguments:
/// 
/// * `par`: The parameter `par` is a mutable reference to a `String` that is passed to the
/// `insert_to_db` function.
/// 
/// Returns:
/// 
/// The function `insert_to_db` is returning a mutable reference to the modified `String` after
/// appending "World" to it.
fn insert_to_db(par: & mut String) -> &String {
    par.push_str(" World");
    return par
}