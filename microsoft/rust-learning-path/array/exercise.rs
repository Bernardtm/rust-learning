#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {
    return if miles > 0 {
        (Age::New, miles)
    } else {
        (Age::Used, miles)
    }
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument 
    let age: (Age, u32) = car_quality(miles);
    if age.0 == Age::Used {
        if roof == true {
             // Call the `println!` macro to show the car order details
             println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", motor, color, miles); 
        } else {
                // Call the `println!` macro to show the car order details
                println!("Prepare a used car: {:?}, {}, Convertible, {} miles\n", motor, color, miles); 
        }
    } else {
        if roof == true {
            // Call the `println!` macro to show the car order details
            println!("Prepare a new car: {:?}, {}, Hard top, {} miles\n", motor, color, miles); 
        } else {
            // Call the `println!` macro to show the car order details
            println!("Prepare a new car: {:?}, {}, Convertible, {} miles\n", motor, color, miles); 
        }
    }
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: age, // todo!("Replace `mileage: miles` with `age` tuple field, call `car_quality()` with `miles` as input argument");
    }
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car = Car {
        color: String::from(colors[0]),
        motor: Transmission::Manual,
        roof: false,
        age: (Age::New, 0),
    };     

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[1]), Transmission::Manual, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    car = car_factory(String::from(colors[2]), Transmission::SemiAuto, false, 565);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    car = car_factory(String::from(colors[3]), Transmission::Automatic, true, 3000);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}
