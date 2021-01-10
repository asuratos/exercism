fn multiple_of_all(num: u32, factors: &[u32]) -> bool {
    factors.iter().filter(|&&n| n != 0).any(|n| num % n == 0)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&i| multiple_of_all(i as u32, factors))
        .sum()
}
