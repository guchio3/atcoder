use proconio::marker::Chars;
use proconio::{fastout, input};

// #[fastout]
// fn main() {
//     input! {
//         mut ss: Chars, t: Chars
//     }
//
//     let mut res: String = String::from("UNRESTORABLE");
//     let ss_len = ss.len();
//     let t_len = t.len();
//
//     let mut all_matched = false;
//     let mut tmp_res = ss.clone();
//     for i in 0..(ss_len - t_len + 1) {
//         let mut matched = true;
//         for j in 0..t_len {
//             if ss[i + j] != '?' && ss[i + j] != t[j] {
//                 matched = false;
//             }
//         }
//         if matched {
//             all_matched = true;
//             tmp_res = ss.clone();
//             for (j, k) in (i..(i + t_len)).into_iter().enumerate() {
//                 tmp_res[k] = t[j];
//             }
//         }
//     }
//
//     if all_matched {
//         for i in 0..ss_len {
//             if tmp_res[i] == '?' {
//                 tmp_res[i] = 'a';
//             }
//         }
//         res = tmp_res.iter().collect();
//     }
//
//     println!("{}", res);
// }

// // ↓ のアルゴリズムではダメ
// // 各 i が start のときをちゃんと調べきれていない
// #[fastout]
// fn main() {
//     input! {
//         mut ss: Chars, t: Chars
//     }
//
//     let ss_len = ss.len();
//     let t_len = t.len();
//
//     let mut matched = false;
//     let mut matched_len = 0;
//     for i in (0..ss_len).rev() {
//         if ss[i] == '?' || ss[i] == t[t_len - matched_len - 1] {
//             matched_len += 1;
//         } else {
//             matched_len = 0;
//         }
//         if matched_len == t_len {
//             for (j, jj) in (i..(i+t_len)).into_iter().enumerate() {
//                 ss[jj] = t[j];
//             }
//             matched = true;
//             break;
//         }
//     }
//
//     let res: String;
//     if matched {
//         for i in 0..ss_len {
//             if ss[i] == '?' {
//                 ss[i] = 'a';
//             }
//         }
//         res = ss.iter().collect();
//     } else {
//         res = String::from("UNRESTORABLE");
//     }
//
//     println!("{}", res);
// }

#[fastout]
fn main() {
    input! {
        mut ss: Chars, t: Chars
    }

    let ss_len = ss.len();
    let t_len = t.len();

    let mut matched = false;
    let mut matched_len = 0;

    let mut i = ss_len - 1;
    loop {
        if ss[i] == '?' || ss[i] == t[t_len - matched_len - 1] {
            matched_len += 1;
        } else {
            i += matched_len;
            matched_len = 0;
        }
        if matched_len == t_len {
            for (j, jj) in (i..(i+t_len)).into_iter().enumerate() {
                ss[jj] = t[j];
            }
            matched = true;
            break;
        }

        if i == 0 {
            break;
        } else {
            i -= 1;
        }
    }

    let res: String;
    if matched {
        for i in 0..ss_len {
            if ss[i] == '?' {
                ss[i] = 'a';
            }
        }
        res = ss.iter().collect();
    } else {
        res = String::from("UNRESTORABLE");
    }

    println!("{}", res);
}
