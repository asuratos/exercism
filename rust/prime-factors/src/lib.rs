pub fn factors(n: u64) -> Vec<u64> {
    let mut p_factors = vec![];
    let mut num = n;

    for i in 2..=(n as f64).sqrt() as u64 {
        while num % i == 0 {
            num = num / i;
            p_factors.push(i)
        }
    }

    if num != 1 {
        p_factors.push(num)
    }

    p_factors

}
