/// The code snippet `pub mod namehelpers { pub fn get_full_name(first: &str, last: &str) -> String {
/// let full_name = format!("{0} {1}", first, last); return full_name; } }` is defining a public module
/// named `namehelpers` that contains a public function `get_full_name`. This function takes two string
/// references `first` and `last` as input, concatenates them with a space in between, and returns the
/// resulting full name as a `String`.
pub mod namehelpers {

    pub fn get_full_name(first: &str, last: &str) -> String  {
        let full_name = format!("{0} {1}", first, last);
    
        return full_name;
    }

}


pub mod database{

}

/// The code snippet `pub mod privatefns { pub fn get_age_plus(age: u16) -> u16 { return age + 5; } }`
/// is defining a public module named `privatefns` that contains a public function `get_age_plus`. This
/// function takes an unsigned 16-bit integer `age` as input and returns another unsigned 16-bit integer
/// which is the result of adding 5 to the input `age`.
pub mod privatefns {
    pub fn get_age_plus(age: u16) -> u16 {
        return age + 5;
    }
}
