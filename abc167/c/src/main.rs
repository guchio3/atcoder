// use std::cmp::max;
use std::cmp::min;
use std::io::stdin;
use std::str::FromStr;

fn read_line<T>() -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_str_as_char_vec() -> Vec<char> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().chars().collect()
}

fn read_str_as_T<T>() -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().unwrap()
}


fn main() {
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];
    let x = in_vec[2];

    let mut in_vecs: Vec<Vec<usize>> = Vec::new();
    for _i in 0..n {
        in_vecs.push(read_line());
    }

    let mut res = -1;
    'outer: for i in 0..(1 << n) {
        let mut a_sum: Vec<usize> = vec![0; m+1];
        let mut c_sum = 0;

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                for k in 1..(m+1) {
                    a_sum[k] += in_vecs[j][k];
                }
                c_sum += in_vecs[j][0];
            }
        }

        for (ii, &a_sum_i) in a_sum.iter().enumerate() {
            if ii == 0 {
                continue;
            }
            if a_sum_i < x {
                continue 'outer;
            }
        }

        if res == -1 {
            res = c_sum as i64;
        } else {
            res = min(res, c_sum as i64);
        }
    }

    println!("{}", res);

}

// fn dfs(sum_money: usize, x: usize, in_vecs: &Vec<Vec<usize>>, best: i64, mut temp_sums: &mut Vec<usize>, index: usize, use_i: usize) -> i64 {
//     if index == in_vecs.len() {
//         for &temp_sum in temp_sums.iter() {
//             if temp_sum < x {
//                 return 1_000_000_000_000;
//             }
//         }
//         return sum_money as i64;
//     }
//     if use_i == 0 {
//         let best = min(best, dfs(sum_money, x, &in_vecs, best, &mut temp_sums, index+1, 0));
//         return min(best, dfs(sum_money, x, &in_vecs, best, &mut temp_sums, index+1, 1));
//     } else {
//         let temp_sum_base = &in_vecs[index];
//         for i in 1..temp_sum_base.len() {
//              temp_sums[i-1] += temp_sum_base[i];
//         }
// 
//         let best = min(best, dfs(sum_money+temp_sum_base[0], x, &in_vecs, best, &mut temp_sums, index+1, 0));
//         let best = min(best, dfs(sum_money+temp_sum_base[0], x, &in_vecs, best, &mut temp_sums, index+1, 1));
// 
//         for i in 1..temp_sum_base.len() {
//              temp_sums[i-1] -= temp_sum_base[i];
//         }
//         return best;
//     }
// }
// 
// fn main() {
//     let in_vec: Vec<usize> = read_line();
//     let n = in_vec[0];
//     let m = in_vec[1];
//     let x = in_vec[2];
// 
//     let mut in_vecs: Vec<Vec<usize>> = Vec::new();
//     for _i in 0..n {
//         in_vecs.push(read_line());
//     }
// 
//     let mut temp_sums: Vec<usize> = vec![0; m];
//     let res = dfs(0, x, &in_vecs, 1_000_000_000_000, &mut temp_sums, 0, 0);
//     let res = min(res, dfs(0, x, &in_vecs, 1_000_000_000_000, &mut temp_sums, 0, 1));
//     
//     if res == 1_000_000_000_000 {
//         println!("{}", -1);
//         return;
//     }
//     println!("{}", res);
// }
