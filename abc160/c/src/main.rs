use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let tmp: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let k = tmp[0];
    let n = tmp[1];
    s.clear();

    stdin().read_line(&mut s).ok();
    let mut a_vec: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a_vec.sort();
    let first_diff = a_vec[0];
    a_vec = a_vec.into_iter().map(|x| x - first_diff).collect();

    let mut a_diff_vec: Vec<i32> = Vec::new();
    let mut tmp_dist = 0;
    for &i_a_vec in a_vec.iter() {
        a_diff_vec.push(i_a_vec - tmp_dist);
        tmp_dist = i_a_vec;
    }
    a_diff_vec.push(k -  a_vec[(n-1) as usize]);
    let max_diff = a_diff_vec.iter().max().unwrap();

    let res = k - max_diff;
    println!("{}", res);
}
