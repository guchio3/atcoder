use proconio::{input, fastout};
// use std::collections::VecDeque;


// fn rec_convert_digit_10_26(res_char_vec: &mut VecDeque<char>, n: f64) {
//     let current_digit = n.log(26.).floor();
//     if current_digit == 0. {
//         res_char_vec.push_front(((n + 96.) as u8) as char);
//     } else {
//         let current_digit_num = (n / (26. as f64).powi(current_digit as i32)).floor();
//         // println!("{} / {} = {}", n, (26. as f64).powi(current_digit as i32), current_digit_num);
//         // println!("{} -> {}", n, n - current_digit_num);
//         rec_convert_digit_10_26(res_char_vec, n - current_digit_num * (26. as f64).powi(current_digit as i32));
//         res_char_vec.push_front(((current_digit_num + 96.) as u8) as char);
//     }
//     
// }
// 
// // #[fastout]
// fn main() {
//     input! {
//         n: f64,
//     }
// 
//     let mut res_char_vec = VecDeque::new();
// 
//     rec_convert_digit_10_26(&mut res_char_vec, n);
// 
//     let res_string: String = res_char_vec.into_iter().collect();
// 
//     println!("{}", res_string);
// }

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut res = String::from("");
    let digit_char;
    if n % 26 == 0 {
        digit_char = 'z';
        n -= 26;
        n /= 26;
    } else {
        digit_char = ((n % 26) as u8 + 96) as char;
        n -= n % 26;
        n /= 26;
    }
    res.push(digit_char);
    while n > 0 {
        let digit_char;
        if n % 26 == 0 {
            digit_char = 'z';
            n -= 26;
            n /= 26;
        } else {
            digit_char = ((n % 26) as u8 + 96) as char;
            n -= n % 26;
            n /= 26;
        }
        res.push(digit_char);
    };

    println!("{}", res.chars().rev().collect::<String>());
}
