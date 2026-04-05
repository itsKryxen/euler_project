pub fn solve() {
    let mut num = 1;
    loop {
        let trinum = gen_triangel_number(num);
        let count = count_factors(trinum);
        if count > 500 {
            println!("{}", trinum);
            break;
        }
        num += 1;
    }
}
fn gen_triangel_number(i: i64) -> i64 {
    i * (i + 1) / 2
}
fn count_factors(n: i64) -> i64 {
    let limit = (n as f64).sqrt() as i64;
    let mut count = 0;

    for i in 1..=limit {
        if n % i == 0 {
            count += if i != n / i { 1 } else { 2 }
        }
    }
    count
}
