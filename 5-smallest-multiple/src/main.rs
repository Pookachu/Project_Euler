fn main() {
    let start: u64 = 1;
    let end: u64 = 20;
    let result = lcm_range(start, end);
    println!("result: {}", result);
}

//greatest common divisor using euclid's algorithm
fn gcd(a: u64, b: u64) -> u64 { 
    if b == 0 {
        a
    } else { 
        gcd(b, a%b)
    }
}

//lowest common multiple
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_range(start: u64, end: u64) -> u64 {
    (start..=end).fold(1, |acc, x| lcm(acc, x))
}