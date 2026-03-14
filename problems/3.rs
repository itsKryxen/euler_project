pub fn solve() {
    let mut number = 600851475143i64;
    let mut largest_prime = -1;
    while number % 2 == 0 {
        largest_prime = 2;
        number /= 2;
    }
    let mut i = 3;
    while i * i <= number {
        while number % i == 0 {
            largest_prime = i;
            number /= i;
        }
        i += 2;
    }
    if number > 2 {
        largest_prime = number;
    }
    println!("answer : {}", largest_prime);
}
