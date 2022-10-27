#[derive(Debug)]
pub enum User<T> {
    Active,
    Inactive,
    Suspended,
    Banned(T)
}

// How to match between Enums.
pub fn user_state(code: u8) -> User<String> {
    match code {
        0 => User::Active,
        1 => User::Inactive,
        2 => User::Suspended,
        _ => User::Banned("Banned".to_string()),
    }
}