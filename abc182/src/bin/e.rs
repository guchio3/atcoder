use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, n: usize, m: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); m]
    }

    // 0 初期
    // 1 block
    // 2 light
    // 3 lighted
    let mut map = vec![vec![0; w]; h];

    for (c_i, d_i) in cd {
        map[c_i - 1][d_i - 1] = 1;
    }

    for (mut a_i, mut b_i) in ab {
        a_i -= 1;
        b_i -= 1;
        map[a_i][b_i] = 2;

        for i in 0..4 {
            let mut temp_a_i = a_i;
            let mut temp_b_i = b_i;
            while (i == 0 && temp_a_i > 0)
                || (i == 1 && temp_a_i < h - 1)
                || (i == 2 && temp_b_i > 0)
                || (i == 3 && temp_b_i < w - 1)
            {
                if i == 0 {
                    temp_a_i -= 1;
                } else if i == 1 {
                    temp_a_i += 1;
                } else if i == 2 {
                    temp_b_i -= 1;
                } else if i == 3 {
                    temp_b_i += 1;
                }

                match map[temp_a_i][temp_b_i] {
                    0 => map[temp_a_i][temp_b_i] = 3,
                    1 => break,
                    2 => break,
                    3 => continue,
                    _ => println!("???"),
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == 2 || map[i][j] == 3 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}

// #[fastout]
// fn main() {
//     input! {
//         h: usize, w: usize, n: usize, m: usize,
//         ab: [(usize, usize); n],
//         cd: [(usize, usize); m]
//     }
//
//     // 0 初期
//     // 1 block
//     // 2 light
//     // 3 lighted
//     let mut map = vec![vec![0; w]; h];
//
//     for (c_i, d_i) in cd {
//         map[c_i - 1][d_i - 1] = 1;
//     }
//
//     for (mut a_i, mut b_i) in ab {
//         a_i -= 1;
//         b_i -= 1;
//         map[a_i][b_i] = 2;
//
//         for i in 0..2 {
//             let mut temp_a_i = a_i;
//             let temp_b_i = b_i;
//             while (i == 0 && temp_a_i > 0) || (i == 1 && temp_a_i < h - 1) {
//                 if i == 0 {
//                     temp_a_i -= 1;
//                 } else {
//                     temp_a_i += 1;
//                 }
//                 match map[temp_a_i][temp_b_i] {
//                     0 => map[temp_a_i][temp_b_i] = 3,
//                     1 => break,
//                     2 => break,
//                     3 => continue,
//                     _ => println!("???"),
//                 }
//             }
//         }
//         for j in 0..2 {
//             let temp_a_i = a_i;
//             let mut temp_b_i = b_i;
//             while (j == 0 && temp_b_i > 0) || (j == 1 && temp_b_i < w - 1) {
//                 if j == 0 {
//                     temp_b_i -= 1;
//                 } else {
//                     temp_b_i += 1;
//                 }
//                 match map[temp_a_i][temp_b_i] {
//                     0 => map[temp_a_i][temp_b_i] = 3,
//                     1 => break,
//                     2 => break,
//                     3 => continue,
//                     _ => println!("???"),
//                 }
//             }
//         }
//     }
//
//     let mut res = 0;
//     for i in 0..h {
//         for j in 0..w {
//             if map[i][j] == 2 || map[i][j] == 3 {
//                 res += 1;
//             }
//         }
//     }
//
//     println!("{}", res);
// }
