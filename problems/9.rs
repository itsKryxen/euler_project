pub fn solve() {
    let max = 1000;
    for i in 1..=max {
        for j in i..max {
            let k = ( i * i + j * j ) as i32;
            let rk = k.isqrt();
            if rk * rk == k && i + j + rk == max {
                println!("a {} b {} c {} , {}", i, j, rk, i * j * rk);
            }
        }
    }
}
