use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let num: u128 = args[1].parse().expect("Failed to parse argument to integer!");

    let biggest_prime: u128 = calculate_biggest_prime_factor(num);
    println!("The biggest prime factor is {biggest_prime}");

}

fn calculate_biggest_prime_factor(mut n: u128) -> u128 {
    let mut max_prime: u128 = 0;

    //Remove all factors of 2
    while n % 2 == 0 {
        max_prime = 2;
        n = n / 2;
    }

    //Remove all factors of 3
    while n % 3 == 0 {
        max_prime = 3;
        n = n / 3;
    }

    //Instead of checking all odd numbers, only numbers of the form 6k ± 1 are tested.
    //This works because all prime numbers greater than 3 follow this pattern.
    let mut i: u128 = 5;
    while i * i <= n {

        while n % i  == 0 {
            max_prime = i;
            n = n / i;  
        }

        while n % (i + 2) == 0 {
            max_prime = i;
            n = n / (i+2);
        }
        i += 6;
    }

    if n > 4 {
        max_prime = n;
    }

    return max_prime
}   