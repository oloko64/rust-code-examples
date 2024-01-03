pub trait LogTerm {
    type Item;

    fn unwrap_log(self) -> Self::Item;
}

impl<T, E> LogTerm for Result<T, E>
where
    T: std::fmt::Display,
    E: std::fmt::Display,
{
    type Item = T;

    fn unwrap_log(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        }
    }
}

// Another implementation

trait CustomPrint<T, E> {
    fn unwrap_or_print(self, msg: &str) -> T;
}

impl<T, E> CustomPrint<T, E> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn unwrap_or_print(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                println!("{}: {:?}", msg, err);
                panic!();
            }
        }
    }
}
