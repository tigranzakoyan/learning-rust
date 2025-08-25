pub mod factorial;


#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial_rec() {
        let result = factorial::factorial_rec(5);
        assert_eq!(result, 2 * 3 * 4 * 5);
    }

    #[test]
    fn test_factorial_loop() {
        let result = factorial::factorial_loop(5);
        assert_eq!(result, 2 * 3 * 4 * 5);
    }
}
