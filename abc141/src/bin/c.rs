use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize, q: usize,
        a: [usize; q]
    }
    let mut successed_num = vec![0; n];
    for a_i in a {
        successed_num[a_i - 1] += 1;
    }
    for i in 0..n {
        if q - successed_num[i] >= k {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
