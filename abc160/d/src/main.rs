use std::cmp;
use std::io::stdin;

fn calc_num_i_j_for_k(x: i32, y: i32, k: i32, n: i32) -> i32 {
    let mut res = 0;
    for i in 1..n {
        for j in i + 1..n + 1 {
            let min_dist = cmp::min(j - i, (x - i).abs() + (j - y).abs() + 1);
            if min_dist == k {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let in_vec: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = in_vec[0];
    let x = in_vec[1];
    let y = in_vec[2];

    let mut reses: Vec<i32> = Vec::new();
    for k in 1..n {
        reses.push(calc_num_i_j_for_k(x, y, k, n));
    }

    for i_res in reses {
        println!("{}", i_res);
    }
}
