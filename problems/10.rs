pub fn solve() {
    let max = 2_000_000;
    let mut isprime = vec![true; max as usize];
    let mut i = 2;
    isprime[0] = false;
    isprime[1] = false;
    while i * i < max {
        if isprime[i] {
            let mut multiple = i * i;
            while multiple < max {
                isprime[multiple] = false;
                multiple += i;
            }
        }
        i += 1;
    }

    let mut sum: i64 = 0;
    for i in 2..max {
        if isprime[i] {
            sum += i as i64;
        }
    }

    println!("{}", sum);
}
