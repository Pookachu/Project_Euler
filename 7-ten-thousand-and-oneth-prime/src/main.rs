fn main() {
    let mut prime_count: u64 = 2;

    //iterating over each number 6k +/- 1 starting at 5 until we hit the 10001st prime
    let mut i: u64 = 5;
    loop {
        // 6k -1
        if is_prime(i) {
            prime_count += 1;
            if prime_count >= 10001 {
                println!("{} is the {}th prime", i, prime_count);
                break
            }
        }
        // 6k +1
        if is_prime(i + 2) {
            prime_count += 1;   
            if prime_count >= 10001 {
                println!("{} is the {}th prime", i + 2, prime_count);
                break
            }
        }
        i += 6;
    }

}

//functionally complete is_prime function
fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => return false, //
        2 | 3 => return true,
        _ => (), // Continue for all other numbers.
    }
    // check if it's immediately divisible by 2 or 3
    if n % 2 == 0 || n % 3 == 0 { return false }

    // iterate from 5 to sqrt(n) by 6 checking if it's divisible by 6k +/- 1
    let divisors = (5..).step_by(6).flat_map(|i| [i, i+2]);

    ! divisors
        .take_while(|i| i * i <= n)
        .any(|i| n % i == 0)
}
