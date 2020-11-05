use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: usize,
        sc: [(usize, usize); m]
    }

    // if n == 1 {
    //     let mut is_ok = true;
    //     let str_i = vec!['0'];
    //     for &(s_i, c_i) in sc.iter() {
    //         if s_i > 1 || ((str_i[s_i - 1] as usize) - ('0' as usize)) != c_i {
    //             is_ok = false;
    //         }
    //     }
    //     if is_ok {
    //         println!("0");
    //         return;
    //     }
    // }

    for i in (10usize.pow(n - 1) - ((n == 1) as usize))..10usize.pow(n) {
        let mut is_ok = true;
        let str_i: Vec<char> = i.to_string().chars().collect();
        for &(s_i, c_i) in sc.iter() {
            if ((str_i[s_i - 1] as usize) - ('0' as usize)) != c_i {
                is_ok = false;
            }
        }
        if is_ok {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
