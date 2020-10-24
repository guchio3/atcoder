use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: Chars,
    }
    let mut mod_res = 0;
    for n_i in n {
        mod_res = (mod_res + (n_i as usize - 48)) % 9;
    }

    if mod_res == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
