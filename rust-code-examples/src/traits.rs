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

trait MyErrorTrait: std::error::Error {
    fn code(&self) -> u32;
}

#[derive(Debug)]
struct MyErrorTraitTest {
    code: u32,
}

impl MyErrorTrait for MyErrorTraitTest {
    fn code(&self) -> u32 {
        self.code
    }
}

impl std::fmt::Display for MyErrorTraitTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyErrorTraitTest: {}", self.code)
    }
}

impl std::error::Error for MyErrorTraitTest {
    fn description(&self) -> &str {
        "MyErrorTraitTest"
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

// ------------------------------------------------------------------------------------------------

fn run_implementations() {
    let test_val = Message::try_from("default").unwrap();

    // This will return an error
    // let test_val = Message::try_from("teste").unwrap();

    implementation(test_val);
    implementation(MessageStruct {
        message: String::from("Test"),
        number: 1,
    });
}

fn implementation(val: impl TestTrait) {
    println!("{}", &val);
    dbg!(&val);
    val.test_error().unwrap();
}

trait MyFrom<T>: Sized {
    fn my_from(t: T) -> Self;
}

trait TestTrait: std::fmt::Display + std::fmt::Debug {
    // This needs to be implemented as Debug is required by unwrap
    type Error: std::fmt::Debug;

    fn test_error(&self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
enum Message {
    Test,
    Default,
}

#[derive(Debug)]
struct MessageStruct {
    message: String,
    number: u32,
}

impl TestTrait for MessageStruct {
    type Error = String;

    fn test_error(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl std::fmt::Display for MessageStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message: {}, Number: {}", self.message, self.number)
    }
}

impl TestTrait for Message {
    type Error = String;

    fn test_error(&self) -> Result<(), Self::Error> {
        // This will return an error if the message enum is Test
        if let Message::Test = self {
            return Err(String::from("Error"));
        }
        Ok(())
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Test => write!(f, "Test"),
            Message::Default => write!(f, "Default"),
        }
    }
}

impl MyFrom<String> for Message {
    fn my_from(s: String) -> Self {
        match s.as_str() {
            "teste" => Message::Test,
            _ => Message::Default,
        }
    }
}

impl TryFrom<&str> for Message {
    // The Error from this trait does not implement Debug and still works with unwrap
    // Needs to investigate why
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "teste" => Ok(Message::Test),
            _ => Ok(Message::Default),
        }
    }
}