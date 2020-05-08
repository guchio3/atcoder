use std::io::stdin;
use std::str::FromStr;
use std::collections::HashMap;

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

fn dfs_dp(a: usize, b: usize, c: usize, place: usize, n: usize, in_vec: &Vec<String>, dp_map: &mut Vec<Vec<Vec<Vec<bool>>>>, res_vec: &mut Vec<char>) -> String {
    if place + 1 == n {
        return "Yes".to_string()
    }

    if in_vec[place] == "AB".to_string() {
        if b > 0 && dp_map[place+1][a+1][b-1][c] {
            res_vec.push('A');
            let res = dfs_dp(a+1, b-1, c, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                // dp_map[place+1][a+1][b-1][c] = false;
                dp_map.insert([place+1][a+1][b-1][c], false);
                res_vec.pop();
            }
        }
        if a > 0 && dp_map[place+1][a-1][b+1][c] {
            res_vec.push('B');
            let res = dfs_dp(a-1, b+1, c, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                dp_map[place+1][a-1][b+1][c] = false;
                res_vec.pop();
            }
        }
    } else if in_vec[place] == "BC".to_string() {
        if c > 0 && dp_map[place+1][a][b+1][c-1] {
            res_vec.push('B');
            let res = dfs_dp(a, b+1, c-1, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                dp_map[place+1][a][b+1][c-1] = false;
                res_vec.pop();
            }
        }
        if b > 0 && dp_map[place+1][a][b-1][c+1] {
            res_vec.push('C');
            let res = dfs_dp(a, b-1, c+1, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                dp_map[place+1][a][b-1][c+1] = false;
                res_vec.pop();
            }
        }
    } else if in_vec[place] == "AC".to_string() {
        if c > 0 && dp_map[place+1][a+1][b][c-1] {
            res_vec.push('A');
            let res = dfs_dp(a+1, b, c-1, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                dp_map[place+1][a+1][b][c-1] = false;
                res_vec.pop();
            }
        }
        if a > 0 && dp_map[place+1][a-1][b][c+1] {
            res_vec.push('C');
            let res = dfs_dp(a-1, b, c+1, n, place+1, in_vec, dp_map, res_vec);
            if res == "Yes".to_string() {
                return "Yes".to_string()
            } else {
                dp_map[place+1][a-1][b][c+1] = false;
                res_vec.pop();
            }
        }
    }

    "No".to_string()
}

fn main() {
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let a = in_vec[1];
    let b = in_vec[2];
    let c = in_vec[3];

    let mut s_vec: Vec<String> = Vec::new();
    for _i in 0..n {
        let mut s = String::new();
        stdin().read_line(&mut s).ok();
        s_vec.push(s.trim().to_string());
    }

    let mut dp_map = ;
    let mut res_vec: Vec<char> = Vec::new();
    let res = dfs_dp(a, b, c, 0, n - 1, &s_vec, &mut dp_map, &mut res_vec);

    println!("{}", res);
    if res == "Yes".to_string() {
        for &c in res_vec.iter() {
            println!("{}", c);
        }
    }
}
