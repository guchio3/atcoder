use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut all = 0;
    for &a_i in a.iter() {
        all ^= a_i;
    }

    // let mut res_str = String::from("");
    // for a_i in a {
    //     res_str = format!("{} {}", res_str, all ^ a_i);
    // }
    let mut reses = vec![];
    for a_i in a {
        reses.push((all ^ a_i).to_string());
    }

    let res_str = reses.join(" ");
    println!("{}", res_str);
}
