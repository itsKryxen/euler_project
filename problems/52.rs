use std::{collections::HashSet, fs};
fn get_words() -> Vec<String> {
    let contents = fs::read_to_string("data/0042_words.txt").unwrap();
    contents.split(',').map(|s| s.trim().to_string()).collect()
}
fn letter_to_value(c: char) -> u32 {
    if !c.is_ascii_uppercase() {
        return 0;
    }
    (c as u8 - b'A' + 1) as u32
}
pub fn solve() {
    let words = get_words();
    let mut map: HashSet<u32> = HashSet::with_capacity(100);
    for i in 0..100 {
        map.insert(i * (i + 1) / 2);
    }
    let mut num = 0;
    for w in words {
        let val = w.chars().fold(0, |v, i| v + letter_to_value(i));
        if map.contains(&val) {
            num += 1;
        }
    }
    println!("answer: {}", num);
}
