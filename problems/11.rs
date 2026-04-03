use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("data/0011_words.txt").unwrap();
    let grid: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let h = grid.len();
    let w = grid[0].len();
    let mut max_product = 0;
    for i in 0..h {
        for j in 0..w {
            if j + 3 < w {
                let prod = grid[i][j] * grid[i][j + 1] * grid[i][j + 2] * grid[i][j + 3];
                max_product = max_product.max(prod);
            }
            if i + 3 < h {
                let prod = grid[i][j] * grid[i + 1][j] * grid[i + 2][j] * grid[i + 3][j];
                max_product = max_product.max(prod);
            }
            if i + 3 < h && j + 3 < w {
                let prod =
                    grid[i][j] * grid[i + 1][j + 1] * grid[i + 2][j + 2] * grid[i + 3][j + 3];
                max_product = max_product.max(prod);
            }
            if i + 3 < h && j >= 3 {
                let prod =
                    grid[i][j] * grid[i + 1][j - 1] * grid[i + 2][j - 2] * grid[i + 3][j - 3];
                max_product = max_product.max(prod);
            }
        }
    }
    println!("answer {}", max_product);
}
