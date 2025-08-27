fn main() {
    let mut prime_count: u64 = 2;

    //iterating over each number 6k +/- 1 starting at 5 until we hit the 10001st prime
    let mut i: u64 = 5;
    loop {
        // 6k -1
        if check_prime(i) {
            prime_count += 1;
            if prime_count >= 10001 {
                println!("{} is the {}th prime", i, prime_count);
                break
            }
        }
        // 6k +1
        if check_prime(i + 2) {
            prime_count += 1;   
            if prime_count >= 10001 {
                println!("{} is the {}th prime", i + 2, prime_count);
                break
            }
        }
        i += 6;
    }

}

//functionally complete check_prime function
//which is unnecessary but cool
fn check_prime(n: u64) -> bool {
    //check if it's 0 or 1
    if n <= 1 {
        return false
    }

    //check if it's 2 or 3
    if n == 2 || n == 3 {
        return true
    }

    //check if it's immediately divisible by 2 or 3
    if n % 2 == 0 || n % 3 == 0 {
        return false
    }

    //iterate from 5 to sqrt(n) by 6 checking if it's divisible by 6k +/- 1
    let mut i: u64 = 5;
    while i <= ((n as f64).sqrt() as u64) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false
        }
        i += 6;
    }

    //if it found nothing n can be divided by cleanly under sqrt(n)
    true
}
