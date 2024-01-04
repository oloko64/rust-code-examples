use std::io::Read;

// Use of Enum errors
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Message(String),
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Message(_) => None,
        }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Message(s) => write!(f, "Message: {s}"),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        Self::Message(s)
    }
}

// Function that returns a Result
fn read_file(path: &str) -> Result<String, MyError> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
