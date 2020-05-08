use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let N: i32 = s.trim().parse().unwrap();
    s.clear(); // $B$3$l$,Bg;v!*!*(B

    stdin().read_line(&mut s).ok();
    let nums: Vec<i32> = s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect();

    let mut register: i32 = 0;
    // 2^0 $B$N0L$,A4$F(B 0 $B$G$"$l$PNI$$(B
    for (i, num) in nums.into_iter().enumerate() {
        // 0 $B$,$"$l$P=*N;(B
        if num == 0 {
            println!("{}", 0);
            return
        }

        // or $B$G(B 0 $B$K$J$k0L$OA4$F(B 2 $B$NG\?t(B
        if i == 0 {
            register = num;
        } else {
            register |= num;
        }
    }

    let mut even_digit_cnt: i32 = 0;
    loop {
        if register & 1 == 1 {
            break;
        } else {
            even_digit_cnt += 1;
            register = register >> 1;
        }
    }

    println!("{}", even_digit_cnt);
}
