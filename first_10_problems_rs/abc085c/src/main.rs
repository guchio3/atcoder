use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let nums: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect();
    let x = nums[0];
    let y = nums[1];

    let mut res = None;

    'outer: for i in 0..x+1 {
        for j in 0..x+1-i {
            if i * 10000 + j * 5000 + (x - i - j) * 1000 == y {
                res = Some((i, j, x - i - j));
                break 'outer;
            }
        }
    }

    let (res_0, res_1, res_2) = res.unwrap_or((-1, -1, -1));
    println!("{} {} {}", res_0, res_1, res_2);
}


// fn main() {
//     let mut s = String::new();
//     stdin().read_line(&mut s).ok();
//     let nums: Vec<i32> = s
//         .trim()
//         .split_whitespace()
//         .map(|x| x.parse().ok().unwrap())
//         .collect();
//     let x = nums[0];
//     let y = nums[1];
// 
//     let max_yen_num = y / 1000;
//     let yen_num_diff = max_yen_num - x;
// 
//     // yen_num_diff が 4 (1000 -> 5000) * i + 9 (1000 -> 10000) * j で割り切れるかを考える
//     let nine_space = yen_num_diff / 9;
//     let nine_space_diff = yen_num_diff % 9;
//     let (four_space, nine_space) = match nine_space_diff {
//         0 => (0, nine_space),
//         3 => (3, nine_space - 1),
//         4 => (1, nine_space),
//         7 => (4, nine_space - 1),
//         8 => (2, nine_space),
//         _ => (-1, -1),
//     };
//     if nine_space < 0 {
//         let four_space = -1;
//     }
// 
//     if four_space == -1 {
//         println!("{} {} {}", -1, -1, -1);
//     } else {
//         println!("{} {} {}", nine_space, four_space, x - nine_space - four_space);
//     }
// }
