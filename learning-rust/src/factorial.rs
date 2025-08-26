pub fn factorial_rec(n: u64) -> u64 {
    if n == 0 {
        return 1
    }
    factorial_rec(n-1) * n
}

pub fn factorial_loop(n: u64) -> u64 {
    let mut sum = 1;
    for m in 2..n+1 {
        sum *= m
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_rec() {
        assert_eq!(factorial_rec(0), 1);
        assert_eq!(factorial_rec(1), 1);
        assert_eq!(factorial_rec(5), 2 * 3 * 4 * 5);
    }

    #[test]
    fn test_factorial_loop() {
        assert_eq!(factorial_loop(0), 1);
        assert_eq!(factorial_loop(1), 1);
        assert_eq!(factorial_loop(5), 2 * 3 * 4 * 5);
    }
}