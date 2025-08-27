fn main() {
    for number_1 in (900..999).rev() {
        for number_2 in (900..999).rev() {

            if palindrome_check(number_1*number_2) {
                println!("{}, {} x {}", number_1*number_2, number_1, number_2);
                return;
            }
        }
    }

}

fn palindrome_check(num: u64) -> bool {
    let mut reverse: u64 = 0; 
    let mut  temp: u64 = num;

    while temp != 0 {
        reverse = (reverse * 10) + (temp % 10);
        temp = temp / 10;
        println!("R: {}, T: {}", reverse, temp);
    }
    reverse == num
}
