// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // 0 = 1
    // 1 = 1
    // 2 = 1 * 2
    // 3 = 1 * 2 * 3
    let mut counter: u32 = 0;
    let mut result: u32 = 1;

    while counter < n {
        counter += 1;
        result = result * counter;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
