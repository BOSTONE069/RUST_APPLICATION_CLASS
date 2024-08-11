#[derive(Debug)]
#[allow(dead_code)]
enum  VehicleColor {
    Red,
    White,
    Black,
    Silver,
    Blue
}

// The `Vehicle` struct represents a vehicle with manufacturer, model, year, and color fields.
// 
// Properties:
// 
// * `manufacturer`: manufacturer: the company that produces the vehicle
// * `model`: The `model` property in the `Vehicle` struct represents the model of the vehicle, such as
// "Civic" for a Honda Civic or "F-150" for a Ford F-150. It is a String type field that holds the
// specific model name of the vehicle.
// * `year`: The `year` property in the `Vehicle` struct represents the year in which the vehicle was
// manufactured. It is of type `u16`, which means it can store unsigned 16-bit integer values ranging
// from 0 to 65,535.
// * `color`: It looks like the `color` field in the `Vehicle` struct is of type `VehicleColor`. Could
// you please provide more information about the `VehicleColor` enum or struct so that I can help you
// further with the definition of the `color` field?
#[derive(Debug)]
struct Vehicle  {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}


impl Vehicle {
    //instance methods
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    pub fn set_year(&mut self, year: u16) {
        self.year = year;
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    //static methods
    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle{
            manufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 1990,
            color: VehicleColor::Blue,
        };
        return new_vehicle;
    }
}


struct Person {
    first_name: String,
    last_name: String,
    birth_year: u32,
    birth_month: u8,
    miles_walked: u32,
}

impl Person {
    fn walk(&mut self, walking: u32) {
        self.miles_walked += walking;
    }
}

#[derive(Debug)]
struct VehicleTuple(String, String, u16);

pub fn create_vehicle_tuple() {
    let myvehicle_tuple = new_vehicletuple();
    println!("{:?}", myvehicle_tuple);
}
fn new_vehicletuple() -> VehicleTuple {
    return VehicleTuple("RangeRover".to_string(), "Landrover".to_string(), 2024);
}


fn new_vehicle() -> Vehicle {
    let mut vehicle = Vehicle {
        manufacturer: "Vocswagen".to_string(),
        model: "Porche".to_string(),
        year: 2024,
        color: VehicleColor::Black,
    };
    vehicle.paint(VehicleColor::White);
    return vehicle;
}

pub fn get_vehicle() {
    let myvehicle: Vehicle = new_vehicle();
    println!("Vehicle manufacturer: {}, model: {}, year: {}, color: {:?}", myvehicle.manufacturer, myvehicle.model, myvehicle.year, myvehicle.color);
}

pub fn create_vehicle() {
    //let my_vehicle = new_vehicle();
    let mut my_vehicle = Vehicle::create_vehicle();
    my_vehicle.set_year(2016);
    my_vehicle.paint(VehicleColor::Silver);
    my_vehicle.set_model("Honda".to_string());
    println!("{:?}", my_vehicle);
}


fn new_person() -> Person {
    let p1 = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        birth_month: 6,
        birth_year: 1986,
        miles_walked: 0,
    };
    return p1;
}

pub fn test_create_person() {
    let mut myperson: Person = new_person();
    myperson.walk(1000);
    myperson.walk(1000);
    println!("First name: {}, Last name: {}, birth month: {}, birth year: {}, miles walked: {}", myperson.first_name, myperson.last_name, myperson.birth_month, myperson.birth_year, myperson.miles_walked);
}