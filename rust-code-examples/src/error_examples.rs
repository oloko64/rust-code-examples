use std::io::Read;

// This can be done with the lib thiserror.
#[derive(Debug)]
enum MyError {
    Error1(String),
    Error2(std::io::Error)
}

// This removes the need to use .map_err(MyError::Error2) after every function that can fail.
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Error2(err)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Error1(msg) => write!(f, "Error: {}", msg),
            MyError::Error2(err) => write!(f, "Error: {}", err),
        }
    }
}

impl std::error::Error for MyError {}

// -----------------------------------------------------------------------------------------

// This is the same as all the code above
// #[derive(Debug, thiserror::Error)]
// enum MyError {
//     #[error("Error: {0}")]
//     Error1(String),
//     #[error("Error: {0}")]
//     Error2(#[from] std::io::Error)
// }

fn read_file() -> Result<String, MyError> {
    let mut file = std::fs::File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
