pub fn solve() {
    let mut v_max = -1;
    let min = 100;
    let max = 999;
    for i in min..=max {
        for j in min..=max {
            let p = i * j;
            if p > v_max && is_palin(p.to_string()) {
                v_max = p;
            }
        }
    }
    println!("answer : {}", v_max);
}
fn is_palin(n: String) -> bool {
    return n == n.chars().rev().collect::<String>();
}
