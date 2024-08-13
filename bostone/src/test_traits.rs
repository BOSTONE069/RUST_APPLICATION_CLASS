struct Person<PetType: Animal + NotDangerous> {
    first_name: String,
    pet: PetType,
}


trait Animal {
    fn make_sounds(&self) -> ();
}

trait NotDangerous {}

struct Dog {}
impl NotDangerous for Dog {}
impl Animal for Dog {
    fn make_sounds(&self) -> (){
        println!("Dog Barked!!!!!");
    }
}

struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
    fn make_sounds(&self) -> (){
        println!("Cat Mooowed!!!!!");
    }
}

pub fn create_person(){
    let pet1 = Dog{};
    let pet2 = Cat{};
    let p1 = Person{first_name: "Bostone".to_string(), pet: pet2};
    p1.pet.make_sounds();
}