mod lib;
use self::lib::*;

fn main() {
    for i in 0..10 {
        println!("{:?}", generate_cpf(Some(i)));
    }
}
