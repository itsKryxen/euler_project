pub fn solve() {
    let count_till = 10_00;
    let mut sum = 0;
    for i in 1..count_till {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("answer : {}", sum);
}
