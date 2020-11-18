use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s_x: f64, s_y: f64, g_x: f64, g_y: f64
    }

    let res = s_x + (g_x - s_x) * s_y / (s_y + g_y);
    println!("{}", res);
}
