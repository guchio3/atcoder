use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        c_h: usize,
        c_w: usize,
        d_h: usize,
        d_w: usize,
        s: [Chars; h],
    }

    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    let mut access_map = vec![vec![false; w]; h];

    let mut cost = -1;
    deque.push_back((c_h - 1, c_w - 1));
    while !deque.is_empty() {
        cost += 1;
        let mut this_loop_coors = Vec::new();
        while !deque.is_empty() {
            let coor = deque.pop_front().unwrap();
            if !access_map[coor.0][coor.1] && s[coor.0][coor.1] == '.' {
                access_map[coor.0][coor.1] = true;
                this_loop_coors.push(coor);
                if coor.0 != 0 {
                    deque.push_back((coor.0 - 1, coor.1));
                }
                if coor.0 != h - 1 {
                    deque.push_back((coor.0 + 1, coor.1));
                }
                if coor.1 != 0 {
                    deque.push_back((coor.0, coor.1 - 1));
                }
                if coor.1 != w - 1 {
                    deque.push_back((coor.0, coor.1 + 1));
                }
            }
        }
        if access_map[d_h - 1][d_w - 1] {
            break;
        }
        for (h_i, w_i) in this_loop_coors {
            if access_map[h_i][w_i] {
                for h_diff in vec![-2, -1, 0, 1, 2] {
                    for w_diff in vec![-2, -1, 0, 1, 2] {
                        if (h_i as i32) + h_diff >= 0
                            && (h_i as i32) + h_diff < h as i32
                            && (w_i as i32) + w_diff >= 0
                            && (w_i as i32) + w_diff < w as i32
                        {
                            let temp_h_i = (h_i as i32 + h_diff) as usize;
                            let temp_w_i = (w_i as i32 + w_diff) as usize;
                            if !access_map[temp_h_i][temp_w_i] && s[temp_h_i][temp_w_i] == '.' {
                                deque.push_back((temp_h_i, temp_w_i));
                            }
                        }
                    }
                }
            }
        }
        // for h_i in 0..h {
        //     for w_i in 0..w {
        //         if access_map[h_i][w_i] {
        //             for h_diff in vec![-2, -1, 0, 1, 2] {
        //                 for w_diff in vec![-2, -1, 0, 1 ,2] {
        //                     if (h_i as i32) + h_diff >= 0 && (h_i as i32) + h_diff < h as i32
        //                         && (w_i as i32) + w_diff >= 0 && (w_i as i32) + w_diff < w as i32 {
        //                             let temp_h_i = (h_i as i32 + h_diff) as usize;
        //                             let temp_w_i = (w_i as i32 + w_diff) as usize;
        //                             if !access_map[temp_h_i][temp_w_i] && s[temp_h_i][temp_w_i] == '.' {
        //                                 deque.push_back((temp_h_i, temp_w_i));
        //                             }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    if access_map[d_h - 1][d_w - 1] {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}
