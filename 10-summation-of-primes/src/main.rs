fn main() {
    let limit = 2_000_000;
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let sum: u64 = is_prime.iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(i, _)| i as u64)
        .sum();

    println!("Sum of all primes below 2 million: {}", sum);
}
