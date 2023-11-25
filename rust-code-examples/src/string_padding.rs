pub fn example() {
    let name1 = "hello";
    let name2 = "world world";

    // pad left
    println!("{:.<20}", name1);

    // pad right
    println!("{:.>20}", name2);

    // pad center
    println!("{:.^20}", name1);
}
