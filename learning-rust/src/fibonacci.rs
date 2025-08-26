// TODO add memoization
pub fn fibonacci_rec(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n
    }
    fibonacci_rec(n-1) + fibonacci_rec(n-2)
}


pub fn fibonacci_loop(n: u64) -> u64 {
    let (mut i, mut j) = (0, 1);
    for _ in 1..n+1 {
        (i, j) = (j, i+j);
    }
    i
}



#[cfg(test)]
mod tests {
    use super::*;

    // index: 0 1 2 3 4 5 6  7  8
    // value: 0 1 1 2 3 5 8 13 21

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(0), 0);
        assert_eq!(fibonacci_rec(1), 1);
        assert_eq!(fibonacci_rec(5), 5);
        assert_eq!(fibonacci_rec(7), 13);
    }    #[test]

    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(0), 0);
        assert_eq!(fibonacci_loop(1), 1);
        assert_eq!(fibonacci_loop(5), 5);
        assert_eq!(fibonacci_loop(7), 13);
    }
}
