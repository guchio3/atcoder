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

fn main() {
    let _n: usize = read_line()[0];
    let a: Vec<usize> = read_line();

    let mut temp_dict: HashMap<usize, usize> = HashMap::new();

    let mut res: usize = 0;
    for (i, &a_i) in a.iter().enumerate() {
        let left: usize = i + 1 + a_i;

        if i + 1 >= a_i {
            let right: usize = i + 1 - a_i;

            match temp_dict.get(&right) {
                Some(cum_num) => {res += cum_num},
                _ => {},
            }
        }

        *(temp_dict.entry(left).or_insert(0)) += 1
    }

    println!("{}", res);
}
