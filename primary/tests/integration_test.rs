#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn lifehooks_test() {
    let mut vec = vec!["1", "2", "3"];
    assert_eq!(vec, ["1", "2", "3"]);
    testsesss::append_arr(&mut vec, "4");
    assert_eq!(vec, ["1", "2", "3", "4"]);
}
