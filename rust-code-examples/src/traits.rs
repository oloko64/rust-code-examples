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
