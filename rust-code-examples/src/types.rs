use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum User<T> {
    Active,
    Inactive,
    Suspended,
    Banned(T)
}

impl<T> Display for User<T> where T: Display {
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

impl<T> Display for UserStruct<T> where T: Display + std::fmt::Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "UserStruct {{ name: {}, age: {}, status: {:?} }}", self.name, self.age, self.status)
    }
}

// How to match between Enums.
pub fn user_state(code: u8) -> User<String> {
    println!("{}", UserStruct{name: "test".to_string(), age: 3, status: User::Banned("test".to_string())});
    match code {
        0 => User::Active,
        1 => User::Inactive,
        2 => User::Suspended,
        _ => User::Banned("Banned".to_string()),
    }
}