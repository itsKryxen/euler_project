pub fn solve() {
    let number = 400_000_0;
    let mut a = 1;
    let mut b = 1;
    let mut sum = 0;
    while b < number {
        let k = b;
        b += a;
        a = k;
        if b % 2 == 0 {
            sum += b;
        }
    }
    println!("answer {}", sum);
}
