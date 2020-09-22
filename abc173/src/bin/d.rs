use proconio::{input, fastout};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    /*
     * $BJ}?K(B
     *
     *  - $BBg$-$$$b$N$+$i=g$KF~$l$F$$$/(B
     *  - $BF~$l$k>l=j$O:GBg$N$b$N$HFsHVL\$KBg$-$$$b$N$N4V(B
     *  - $B:GBg$N$b$N$,(B 2 $B$D0J>e$"$k>l9g$O$=$N4V$KF~$l$k(B
     *  - $B:GBg$N$b$N$,(B 1 $B$D$7$+$J$/!"$^$?FsHVL\$N$b$N$,(B 2 $B$D0J>e$"$k>l9g$OFsHVL\$N$b$N$N4V$KF~$l$k(B
     *  ($BFsHVL\$N$b$N$OO"B3$7$F$$$k$O$:!#$?$@$7!"(B)
     *
     *  $B$F$+(B greedy $B$K2r$1$k!)(B
     *  priority queue $B$K(B pair $B$N:G>.CM$rF~$l$k(B
     *   $B"*(B $B9b$$$b$N$+$i=g$K(B pop $B$7!"(Bres+=1 $B$7!"(Bnew pair $B$N:G>.CM(B*2 $B$r(B push
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
