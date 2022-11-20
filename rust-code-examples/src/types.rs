use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
#[derive(Debug)]
pub enum User<T> {
    Active,
    Inactive,
    Suspended,
    Banned(T),
}

impl<T> Display for User<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            User::Active => write!(f, "Active"),
            User::Inactive => write!(f, "Inactive"),
            User::Suspended => write!(f, "Suspended"),
            User::Banned(reason) => write!(f, "Banned: {}", reason),
        }
    }
}

struct UserStruct<T> {
    name: String,
    age: u8,
    status: User<T>,
}

impl<T> Display for UserStruct<T>
where
    T: Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "UserStruct {{ name: {}, age: {}, status: {:?} }}",
            self.name, self.age, self.status
        )
    }
}

// How to match between Enums.
pub fn user_state(code: u8) -> User<String> {
    println!(
        "{}",
        UserStruct {
            name: "test".to_string(),
            age: 3,
            status: User::Banned("test".to_string())
        }
    );
    match code {
        0 => User::Active,
        1 => User::Inactive,
        2 => User::Suspended,
        _ => User::Banned("Banned".to_string()),
    }
}


// Is very important to define your data structure correctly, this saves a lot of time from doing a lot of refactoring.
pub struct Configuration {
    data: String,
    last_modified: i64,
    modified_by: String,
}

impl Configuration {
    // This is a constructor, it is used to create a new instance of the struct.
    // Generally is not a good practice to require a lot of parameters here. In this case only 1 is required.
    #[must_use = "This function returns a new instance of the configuration"]
    pub fn new(config_path: &str) -> Configuration {
        let data = Self::read_configuration(config_path)
            .expect("Failed to read configuration file");
        let last_modified = Self::get_last_modified(config_path)
            .expect("Failed to get last modified time");
        let modified_by = Self::get_modified_by(config_path).expect("Failed to get modified by");
        
        // After all the data is collected, we can create a new instance of the struct and return it.
        Configuration {
            data,
            last_modified,
            modified_by,
        }
    }

    // This is a function, it is used to read the configuration file. Remember to name it properly.
    // It is makes coding a lot easier when you build the shell of the function first, the implementation can be done later.
    // Define the parameters and its types, and the return type.
    // In this case, the function returns a Result, which is a type that can be either a Result of a String or an io::Error.
    fn read_configuration(path: &str) -> Result<String, std::io::Error> {
        todo!("Read configuration from file")
        // Read the file and parse it to the desired format.
        // Return the data of the configuration or an io Error.
    }

    // In this case I dont know yet if we can define one type of error so we can make it dynamic.
    fn get_last_modified(path: &str) -> Result<i64, Box<dyn Error>> {
        todo!("Read last modified from file")
        // Find a way to get the last modified date of the file.
        // Return the last modified date or some type Error.
    }

    // Wen creating the shell of the function you can use the todo! macro to make it easier to find the functions that need to be implemented.
    // In this cases you don't need the ownership of the path, so you can use a reference to it, improving the performance of the function and using less memory.
    fn get_modified_by(path: &str) -> Result<String, Box<dyn Error>> {
        todo!("Read modified by from file")
        // Find a way to get the last modified by of the file.
        // Return the last modified by or some type Error.
    }
}
fn example() {
    // This is a usage example, seeing how to use the Configuration struct.
    let _config = Configuration::new("path");
}
