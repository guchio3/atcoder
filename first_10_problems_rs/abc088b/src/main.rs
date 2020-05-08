use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let _n: i32 = s.trim().parse().ok().unwrap();
    s.clear();

    stdin().read_line(&mut s).ok();
    let mut nums: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect();

    nums.sort();

    // let sum_i: i32 = nums.iter().step_by(2).into_iter().sum::<i32>();
    let sum_i: i32 = nums
        .iter()
        .enumerate()
        .filter(|&(i, _x)| i % 2 == 0)
        .into_iter()
        .map(|(_i, x)| x)
        .sum::<i32>();
    let diff: i32 = (nums.iter().sum::<i32>() - sum_i * 2).abs();

    println!("{}", diff)
}
