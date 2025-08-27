use array::SERIES;
mod array;

fn main() {
    let mut max_product = 0u64;
    for i in 0..=SERIES.len() - 13 {
        let window = &SERIES[i..i + 13];
        if window.contains(&0) {
            continue
        }

        let product = window.iter().product();

        if product > max_product {
            max_product = product;
        }
    }
    println!("{}", max_product)
}
