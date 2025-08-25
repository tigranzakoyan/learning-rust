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