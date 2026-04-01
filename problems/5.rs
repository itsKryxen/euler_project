pub fn solve() {
    let min = 1;
    let max = 20;
    let mut number = max;
    loop {
        let mut found = true;
        for i in min..=max {
            if number % i != 0 {
                found = false;
                break;
            }
        }
        if found {
            break;
        }
        number += 1;
    }
    println!("answer : {}", number);
}
