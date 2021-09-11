use rust_iterators::{pairing, frequency_count, calendar};


fn main() {
    println!("pairing: {:?}\n", pairing(&vec![1, 2], &vec![3, 4]));
    println!("frequency_count: {:?}\n", frequency_count(&vec![10, 3, 10]));
    calendar(2022, 9);
}
