pub fn solve() {
    let numb = 100;
    let num_sum = numb * (numb + 1) / 2;
    let num_each_sq = numb * (numb + 1) * (2 * numb + 1) / 6;
    let answer = num_sum * num_sum - num_each_sq;
    println!("answer : {}", answer);
}
