fn main() {
    let mut sum_of_squares: u64 = 0;
    let mut square_of_sums: u64 = 0;
    for i in 1..101 {
        sum_of_squares += i*i;
        square_of_sums += i;
    }
    square_of_sums *= square_of_sums;

    let result = square_of_sums - sum_of_squares;
    println!("result: {}", result);
}
