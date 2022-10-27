pub fn second_number(num: i32) -> i32 {
    num * 2
}

// Test the second_number function.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_by_two() {
        assert_eq!(second_number(2), 4);
    }
}
