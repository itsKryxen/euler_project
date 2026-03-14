pub fn solve() {
    let need = 10001;
    let mut current = 0;
    let mut current_number = 2;
    while current < need {
        let mut is_prime = true;
        for i in 2..=(current_number as f64).sqrt() as i64 {
            if current_number % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            current += 1;
        }
        current_number += 1;
    }
    println!("answer : {}", current_number - 1);
}
