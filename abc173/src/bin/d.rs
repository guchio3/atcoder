use proconio::{input, fastout};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    /*
     * 方針
     *
     *  - 大きいものから順に入れていく
     *  - 入れる場所は最大のものと二番目に大きいものの間
     *  - 最大のものが 2 つ以上ある場合はその間に入れる
     *  - 最大のものが 1 つしかなく、また二番目のものが 2 つ以上ある場合は二番目のものの間に入れる
     *  (二番目のものは連続しているはず。ただし、)
     *
     *  てか greedy に解ける？
     *  priority queue に pair の最小値を入れる
     *   → 高いものから順に pop し、res+=1 し、new pair の最小値*2 を push
     *
     * */
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();

    let mut res = 0;
    let mut bh = BinaryHeap::new();

    for a_i in a {
        if bh.len() == 0 {
            bh.push(a_i);
        } else {
            let confort = bh.pop();
            if let Some(confort_cnt) = confort {
                res += confort_cnt;
            } else {
                panic!("None has poped from the bh.")
            }
            bh.push(a_i);
            bh.push(a_i);
        }
    }

    println!("{}", res);
}
