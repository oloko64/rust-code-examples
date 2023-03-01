
/// In this case the function returns a generic type
pub fn important_num_generator(num: f64) -> f64 {
    num * 2.0
}

/// So this function can be called with a wrong value, with a valid type
pub fn important_num_consumer(num: f64) {
    println!("important_num_consumer: {}", num);
}

// --------------------------------------------

// Defining a new type
pub struct ImportantNumber(f64);

/// In this case the function returns a new type
pub fn important_num_generator_new_type(num: f64) -> ImportantNumber {
    ImportantNumber(num * 2.0)
}

/// So this function cannot be called with a wrong value, only a value of the new type
/// You can use the `ImportantNumber(num): ImportantNumber` to get the underlying value, so you can use it without calling `num.0`
/// 
/// This is a good way to avoid errors
pub fn important_num_consumer_new_type(ImportantNumber(num): ImportantNumber) {
    println!("important_num_consumer: {}", num);
}