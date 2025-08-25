pub fn factorial_rec(n: u64) -> u64 {
    if n == 1 {
        return n
    }
    factorial_rec(n-1) * n
}

pub fn factorial_loop(n: u64) -> u64 {
    let mut sum = 1;
    for m in 1..n+1 {
        sum *= m;
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_rec() {
        let result = factorial_rec(5);
        assert_eq!(result, 2 * 3 * 4 * 5);
    }

    #[test]
    fn test_factorial_loop() {
        let result = factorial_loop(5);
        assert_eq!(result, 2 * 3 * 4 * 5);
    }
}