use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n]
    }

    let mut exist = vec![vec![false; 3]; 3];
    for b_i in b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b_i {
                    exist[i][j] = true;
                }
            }
        }
    }

    let mut is_ok3 = true;
    let mut is_ok4 = true;
    for i in 0..3 {
        let mut is_ok = true;
        let mut is_ok2 = true;
        for j in 0..3 {
            if !exist[i][j] {
                is_ok = false;
            }
            if !exist[j][i] {
                is_ok2 = false;
            }
        }
        if !exist[i][i] {
            is_ok3 = false;
        }
        if !exist[i][3-i-1] {
            is_ok4 = false;
        }
        if is_ok {
            println!("Yes");
            return;
        }
        if is_ok2 {
            println!("Yes");
            return;
        }
    }
    if is_ok3 {
        println!("Yes");
        return;
    } else if is_ok4 {
        println!("Yes");
        return;
    } else{
        println!("No");
        return;
    }
}
