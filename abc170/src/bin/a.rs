use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        in_vec: [usize; 5],
    }

    let mut res_i = 0;
    for i in 1..=5 {
        if in_vec[i - 1] == 0 {
            res_i = i;
        }
    }

    println!("{}", res_i);
}
