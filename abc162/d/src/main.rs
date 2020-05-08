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
    let in_vec: Vec<i32> = read_line();
    let n = in_vec[0];
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let s: Vec<char> = s.trim().to_string().chars().collect();

    // $BA4It0c$&$7Ey4V3V$G$J$$AH$N?t$r5a$a$k(B
    let mut abc_map_1: HashMap<char, i64> = HashMap::new();
    abc_map_1.insert('R', 0);
    abc_map_1.insert('G', 0);
    abc_map_1.insert('B', 0);
    let mut abc_map_2: HashMap<char, i64> = HashMap::new();
    abc_map_2.insert('R', 0);
    abc_map_2.insert('G', 0);
    abc_map_2.insert('B', 0);
    let mut abc_map_3: HashMap<char, i64> = HashMap::new();
    abc_map_3.insert('R', 0);
    abc_map_3.insert('G', 0);
    abc_map_3.insert('B', 0);

    for i in 0..n {
        *(abc_map_3.get_mut(&s[i as usize]).unwrap()) += 1;
    }

    let mut res: i64 = 0;
    // 2 $BHVL\(B
    for i in 0..n {
        // $B0lHV:8(B
        // $B99?7(B
        *abc_map_2.get_mut(&s[i as usize]).unwrap() += 1;
        *abc_map_3.get_mut(&s[i as usize]).unwrap() -= 1;
        if s[i as usize] == 'R' {
            res += abc_map_1[&'G'] * abc_map_3[&'B'];
            res += abc_map_1[&'B'] * abc_map_3[&'G'];
        } else if s[i as usize] == 'G' {
            res += abc_map_1[&'R'] * abc_map_3[&'B'];
            res += abc_map_1[&'B'] * abc_map_3[&'R'];
        } else {
            res += abc_map_1[&'R'] * abc_map_3[&'G'];
            res += abc_map_1[&'G'] * abc_map_3[&'R'];
        }

        let mut start = i - 1;
        if start < 0 {
            start = 0;
        }
        for j in start..i {
            *abc_map_1.get_mut(&s[j as usize]).unwrap() += 1;
            *abc_map_2.get_mut(&s[j as usize]).unwrap() -= 1;
            if s[j as usize] == 'R' {
                res += abc_map_2[&'G'] * abc_map_3[&'B'];
                res += abc_map_2[&'B'] * abc_map_3[&'G'];
            } else if s[j as usize] == 'G' {
                res += abc_map_2[&'R'] * abc_map_3[&'B'];
                res += abc_map_2[&'B'] * abc_map_3[&'R'];
            } else {
                res += abc_map_2[&'R'] * abc_map_3[&'G'];
                res += abc_map_2[&'G'] * abc_map_3[&'R'];
            }
        }
    }

    let mut dup = 0;

    for win_size in 0..(n - 3) / 2 + 1 {
        let mut i = 0;
        while i + win_size * 2 + 2 < n {
            if (&s[i as usize] != &s[(i + win_size + 1) as usize])
                && (&s[(i + win_size + 1) as usize] != &s[(i + win_size * 2 + 2) as usize])
                && (&s[i as usize] != &s[(i + win_size * 2 + 2) as usize])
            {
                dup += 1;
            }
            i += 1;
        }
    }

    println!("{}", res - dup);
}
