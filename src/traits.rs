/// The `trait Describe` defines a trait in Rust that includes a method signature `describe_user` which
/// takes a reference to `self` and returns a `String`. This trait can be implemented for any type,
/// allowing those types to provide an implementation for the `describe_user` method. In this case, the
/// `Person` struct implements the `Describe` trait by providing its own implementation for the
/// `describe_user` method.
trait Describe {
    fn describe_user(&self)-> String;
}

struct Person {
    name: String,
    age: i32
}

impl Describe for Person {
    fn describe_user(&self) -> String{
        format!("Username : {}, Age {}", self.name, self.age)
    }

}


pub fn traits_person (){
    let person = Person{
        name: String::from("Kenn"),
        age : 40
    };

    println!("{}", person.describe_user())
}