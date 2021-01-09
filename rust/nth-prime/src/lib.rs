fn is_prime(n: u32) -> bool {
    let lim = (n as f64).sqrt() as u32;

    for i in 2..=lim {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut current = 2;
    let mut primes_found = 0;

    loop {
        if is_prime(current) {
            primes_found += 1;
        }
        if primes_found-1 == n {
            return current;
        }

        current += 1;
    }
}
