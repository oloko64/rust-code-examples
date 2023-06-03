use std::collections::HashMap;

/// A version of the fibonacci function that uses memoization.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// let mut mem = HashMap::new();
///
/// assert_eq!(fib(0, &mut mem), 0);
/// assert_eq!(fib(10, &mut mem), 55);
/// ```
fn fib_memo(n: u8, mem: &mut HashMap<u8, u128>) -> u128 {
    if n <= 1 {
        return n as u128;
    } else if mem.contains_key(&n) {
        return mem[&n];
    } else {
        let result = fib_memo(n - 1, mem) + fib_memo(n - 2, mem);
        mem.insert(n, result);
        return result;
    }
}

/// A version of the fibonacci function that does not use memoization.
fn fib(n: u8) -> u128 {
    if n <= 1 {
        return n as u128;
    }

    fib(n - 1) + fib(n - 2)
}
