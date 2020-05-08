use std::io::stdin;

fn calc_ten_digit_num_sum(mut num: i32) -> i32 {
    let mut res = 0;
    while num > 0 {
        res += num % 10;
        num /= 10;
    }
    res
}

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();

    let nums: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect();

    let n = nums[0];
    let a = nums[1];
    let b = nums[2];

    let mut total_sum = 0;

    // logic
    for i in 1..n+1 {
        let temp_sum = calc_ten_digit_num_sum(i);
        if a <= temp_sum && temp_sum <= b {
            total_sum += i;
            // total_sum += temp_sum;
        }
    }

    println!("{}", total_sum);
}
