fn main() {
    let sum = 1000;

    for a in 1..sum / 3 {
        for b in a + 1..sum / 2 {
            let c = sum - a - b;
            if a*a + b*b == c*c {
                println!("Found triplet: a={}, b={}, c={}", a, b, c);
                println!("Product: {}", a * b * c);
                return;
            }
        }
    }
}
