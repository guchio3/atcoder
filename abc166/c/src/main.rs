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
    let in_vec: Vec<usize> = read_line();
    let n = in_vec[0];
    let m = in_vec[1];

    let h: Vec<usize> = read_line();

    let mut link_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for _i in 0..m {
        let in_vec: Vec<usize> = read_line();
        let a = in_vec[0];
        let b = in_vec[1];
        if link_map.contains_key(&(a - 1)) {
            (*(link_map.get_mut(&(a - 1)).unwrap())).push(b - 1);
        } else {
            link_map.insert(a - 1, vec![b - 1]);
        }
        if link_map.contains_key(&(b - 1)) {
            (*(link_map.get_mut(&(b - 1)).unwrap())).push(a - 1);
        } else {
            link_map.insert(b - 1, vec![a - 1]);
        }
    }

    let mut res = 0;
    'outer: for key in 0..n {
        if link_map.contains_key(&key) {
            let key_hight = h[key];
            let val = link_map.get(&key).unwrap();
            for &val_i in val.iter() {
                let val_height = h[val_i];
                if key_hight <= val_height {
                    continue 'outer;
                }
            }
            res += 1;
        } else {
            res += 1;
        }
    }

    println!("{}", res);
}
