use std::fs;

use crate::utils::BigInt;

pub fn solve() {
    let contents = fs::read_to_string("data/0013_words.txt").unwrap();

    let numbers: Vec<BigInt> = contents
        .lines()
        .map(|x| BigInt::from_str(x).unwrap())
        .collect();
    if let Some(first) = numbers.first() {
        let mut sum = first.clone();
        for i in 1..numbers.len() {
            sum = sum.add(&numbers[i]);
        }
        //only first 10 digits are asked
        println!("{}", &sum.to_string()[..10]);
    }
}
