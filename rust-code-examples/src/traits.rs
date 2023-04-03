struct Vehicle {
    price: u32,
    weight: u32,
    model: String,
}
trait VehicleTrait {
    fn get_price(&self) -> u32;
    fn get_weight(&self) -> u32;
    fn get_model(&self) -> String;
}
struct Truck {
    vehicle: Vehicle,
    capacity: u32,
}

impl Truck {
    fn new(price: u32, weight: u32, model: String, capacity: u32) -> Truck {
        Truck {
            vehicle: Vehicle {
                price,
                weight,
                model,
            },
            capacity,
        }
    }
}

// Creates a trait bound between Vehicle and Truck.
trait TruckTrait: VehicleTrait {
    fn get_capacity(&self) -> u32;
}

impl VehicleTrait for Truck {
    fn get_price(&self) -> u32 {
        self.vehicle.price
    }

    fn get_weight(&self) -> u32 {
        self.vehicle.weight
    }

    fn get_model(&self) -> String {
        self.vehicle.model.clone()
    }
}

impl TruckTrait for Truck {
    fn get_capacity(&self) -> u32 {
        self.capacity
    }
}

pub fn test_trait() {
    let truck = Truck::new(100, 200, "truck".to_string(), 300);

    print_trait(&truck);
    print_trait_dyn(Box::new(truck));
}

// Now we can use the trait bound to print the values of the truck, heres is using the impl keyword
// This increases the code size but at no runtime cost.
fn print_trait(truck: &impl TruckTrait) {
    println!("Price: {}", truck.get_price());
    println!("Weight: {}", truck.get_weight());
    println!("Model: {}", truck.get_model());
    println!("Capacity: {}", truck.get_capacity());
}

// This is using the dyn keyword, this increases the runtime cost but not the code size.
fn print_trait_dyn(truck: Box<dyn TruckTrait>) {
    println!("Price: {}", truck.get_price());
    println!("Weight: {}", truck.get_weight());
    println!("Model: {}", truck.get_model());
    println!("Capacity: {}", truck.get_capacity());
}

// ------------------------------------------------------------------------------------------------

trait Human {
    fn eat(&self);
    fn sleep(&self);
}

// This is another way of implementing trait, without using blanket implementation
// trait SuperHuman: Human {
//     fn fly(&self);
// }

// impl Human for Superman {
//     fn eat(&self) {
//         println!("Human eats");
//     }

//     fn sleep(&self) {
//         println!("Human sleeps");
//     }
// }

trait SuperHuman {
    fn fly(&self);
}
// This is called blanket implementation
impl<U: SuperHuman> Human for U {
    fn eat(&self) {
        println!("Human eats");
    }

    fn sleep(&self) {
        println!("Human sleeps");
    }
}

struct Superman;

impl SuperHuman for Superman {
    fn fly(&self) {
        println!("Superman flies");
    }
}

fn main_two() {
    let superman = Superman;
    superman.eat();
    superman.sleep();
    superman.fly();
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct MyError {
    code: u32,
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        "MyError"
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyError: {}", self.code)
    }
}

fn my_error_test() -> Result<(), Box<dyn std::error::Error>> {
    if true {
        return Err(MyError { code: 1 }.into());
    } else {
        return Ok(());
    }
}

// ------------------------------------------------------------------------------------------------

trait MyErrorTrait {
    fn code(&self) -> u32;
}

struct MyErrorTraitTest {
    code: u32,
}

impl MyErrorTrait for MyErrorTraitTest {
    fn code(&self) -> u32 {
        self.code
    }
}

impl Into<Box<dyn MyErrorTrait>> for MyErrorTraitTest {
    fn into(self) -> Box<dyn MyErrorTrait> {
        Box::new(self)
    }
}

fn my_error_trait_test() -> Result<(), Box<dyn MyErrorTrait>> {
    if true {
        return Err(MyErrorTraitTest { code: 1 }.into());
    } else {
        return Ok(());
    }
}
