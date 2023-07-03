use std::io::Error;

// Example
// https://github.com/danburkert/snazzy/tree/master
// https://docs.rs/prost-build/0.11.9/prost_build/

fn main() -> Result<(), Error> {
    prost_build::compile_protos(&["src/persons.proto"], &["src/"])?;
    Ok(())
}
