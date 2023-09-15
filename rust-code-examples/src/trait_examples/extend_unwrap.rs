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