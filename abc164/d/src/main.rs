use std::io::stdin;
// use std::collections::HashMap;

fn main() {
    let mut s: String = String::new();
    stdin().read_line(&mut s).ok();
    let s: Vec<char> = s.trim().chars().collect();

    let mut counts: Vec<usize> = vec![0; 2019];
    counts[0] += 1;

    // s を操作していき、mod 2019 を count として保存していく
    let mut base = 1;
    let mut temp_mod = 0;
    for &s_i in s.iter().rev() {
        temp_mod = (temp_mod + (s_i as usize - 48) * base) % 2019;
        base = (base * 10) % 2019;
        if s_i != '0' {
            counts[temp_mod] += 1;
        }
    }

    let mut res = 0;
    for counts_i in counts {
        if counts_i > 1 {
            res += counts_i * (counts_i - 1) / 2;
        }
    }
    println!("{}", res);
}


// fn main() {
//     let mut s: String = String::new();
//     stdin().read_line(&mut s).ok();
//     let s: Vec<char> = s.trim().chars().collect();
// 
//     // 後ろから走査していく
//     // 各注目数について、剰余を出して、一つ左の剰余を足して... を繰り返していく
// 
//     let mut res = 0;
//     let mut dp_dict: HashMap<usize, usize> = HashMap::new();
//     for i_upper in 0..s.len() {
//         let mut temp_mod = 0;
//         let mut temp_dp_dict: HashMap<usize, usize> = HashMap::new();
//         for i_lower in i_upper..s.len() {
//             // println!("{}, {}", s[i_lower], s[i_lower] as usize);
//             temp_mod = (temp_mod * 10 + (s[i_lower] as usize - 48)) % 2019;
// 
//             // temp_mod = (temp_mod + (s[i_upper] as usize) * base);
// 
//             // let temp_str: String = s[i_upper..i_lower + 1].iter().collect();
//             // // println!("{}, {}", temp_str, temp_str.parse::<usize>().unwrap() % 2019);
//             // // temp_mod = (temp_mod + temp_str.parse::<usize>().unwrap() % 2019) % 2019;
//             // temp_mod = temp_str.parse::<usize>().unwrap() % 2019;
//             // // println!(" === {} === ", temp_mod);
// 
//             if temp_mod == 0 {
//                 if dp_dict.contains_key(&i_lower) {
//                     res += dp_dict.get(&i_lower).unwrap();
//                     for (k, v) in temp_dp_dict.iter_mut() {
//                         *v += 1;
//                     }
//                     break;
//                 }
//                 res += 1;
//                 temp_dp_dict.insert(i_lower, 0);
//                 for (k, v) in temp_dp_dict.iter_mut() {
//                     *v += 1;
//                 }
//             }
//         }
//         for (k, v) in temp_dp_dict.iter() {
//             dp_dict.insert(*k, *v);
//         }
//     }
// 
//     println!("{}", res);
// }
