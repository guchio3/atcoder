use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }
    let mut res = 0;

    for i in 0..((2 as usize).pow(h as u32)) {
        for j in 0..((2 as usize).pow(w as u32)) {
            let mut temp_c = c.clone();
            for ii in 0..h {
                if ((i+1) & (1 << ii)) > 0 {
                    for i_j in 0..w {
                        temp_c[ii][i_j] = 'r';
                    }
                }
            }
            for jj in 0..w {
                if ((j+1) & (1 << jj)) > 0 {
                    for j_i in 0..h {
                        temp_c[j_i][jj] = 'r';
                    }
                }
            }

            // cnt #
            let mut temp_cnt_k = 0;
            for cnt_i in 0..h {
                for cnt_j in 0..w {
                    if temp_c[cnt_i][cnt_j] == '#' {
                        temp_cnt_k += 1;
                    }
                }
            }
            if temp_cnt_k == k {
                res += 1;
            }
        }
    }

    println!("{:?}", res);
}
