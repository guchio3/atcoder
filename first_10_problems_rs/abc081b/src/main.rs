use std::io::stdin;

fn main() {
    let mut s = String::new();

    stdin().read_line(&mut s).ok();
    let N: i32 = s.trim().parse().unwrap();
    s.clear(); // これが大事！！

    stdin().read_line(&mut s).ok();
    let nums: Vec<i32> = s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect();

    let mut register: i32 = 0;
    // 2^0 の位が全て 0 であれば良い
    for (i, num) in nums.into_iter().enumerate() {
        // 0 があれば終了
        if num == 0 {
            println!("{}", 0);
            return
        }

        // or で 0 になる位は全て 2 の倍数
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
