use proconio::input;
use proconio::marker::Chars;
// use std::collections::VecDeque;

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    let mut receive_s = vec![];
    for s_i in s {
        if s_i == 'x' {
            if receive_s.len() >= 2
                && receive_s[receive_s.len() - 2] == 'f'
                && receive_s[receive_s.len() - 1] == 'o'
            {
                receive_s.pop();
                receive_s.pop();
            } else {
                receive_s.push(s_i);
            }
        } else {
            receive_s.push(s_i);
        }
    }

    println!("{}", receive_s.len());
}

// fn main() {
//     input! {
//         n: usize,
//         s: Chars
//     }
//
//     let mut tmp: VecDeque<char> = VecDeque::new();
//     let mut res_minus = 0;
//
//     for s_i in s {
//         if s_i == 'f' {
//             tmp.push_back('f');
//         } else if s_i == 'o' {
//             if tmp.len() > 0 && tmp[tmp.len() - 1] == 'f' {
//                 tmp.push_back('o');
//             } else {
//                 tmp = VecDeque::new();
//             }
//         } else if s_i == 'x' {
//             if tmp.len() > 1 && tmp[tmp.len() - 1] == 'o' {
//                 tmp.pop_back();
//                 tmp.pop_back();
//                 res_minus += 3;
//             } else {
//                 tmp = VecDeque::new();
//             }
//         } else {
//             tmp = VecDeque::new();
//         }
//     }
//
//     println!("{}", n - res_minus);
// }
