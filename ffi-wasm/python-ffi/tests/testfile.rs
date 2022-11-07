use string_sum;

// This test is currently failing when building with `cargo test`:
#[test]
fn it_works() {
    let result = string_sum::sum_as_string(2, 2);
    assert_eq!(result.ok(), Some("4".to_string()));
}