use std::io::stdin;
use std::str::FromStr;
use std::cmp::max;

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

fn dfs(i: usize, n_info: (usize, usize), m:usize, state: &mut Vec<usize>, reps: &mut Vec<(usize, usize, usize, usize)>) -> usize {
    let mut temp = 0;

    if n_info.0 < n_info.1 {
        for temp_i in i..=m {
            state.push(temp_i);
            temp = max(temp, dfs(temp_i, (n_info.0 + 1, n_info.1), m, state, reps));
            state.pop().unwrap();
        }
    } else if n_info.0 == n_info.1 {
        for &(a, b, c, d) in reps.iter() {
            if state[b-1] - state[a-1] == c {
                temp += d;
            }
        }
    } else {
        panic!("temp");
    }
    temp
}

fn main() {
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];
    let q = in_vec[2];

    // 一旦 10 重ループで求めてみる
    let mut reps = Vec::<(usize, usize, usize, usize)>::new();
    for _i in 0..q {
        let in_vec: Vec<usize> = read_line();
        reps.push((in_vec[0], in_vec[1], in_vec[2], in_vec[3]));
    }

    let mut state: Vec<usize> = Vec::new();
    let res = dfs(1, (0, n), m, &mut state, &mut reps);

    println!("{}", res);

    // let mut res = 0;
    // for l0 in 1..m+1 {
    //     for l1 in l0..=m {
    //         for l2 in l1..=m {
    //             for l3 in l2..=m {
    //                 for l4 in l3..=m {
    //                     for l5 in l4..=m {
    //                         for l6 in l5..=m {
    //                             for l7 in l6..=m {
    //                                 for l8 in l7..=m {
    //                                     for l9 in l8..=m {
    //                                         let mut temp = 0;
    //                                         let state = vec![l0, l1, l2, l3, l4, l5, l6, l7, l8, l9];
    //                                         for &(a, b, c, d) in reps.iter() {
    //                                             if state[b-1] - state[a-1] == c {
    //                                                 temp += d;
    //                                             }
    //                                         }
    //                                         res = max(res, temp);
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("{}", res);
}
