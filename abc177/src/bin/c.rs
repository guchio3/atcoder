use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    // let a_mod_sum = a.iter().fold(0, |a, b| (a + b) % MOD);
    // println!("{}", a_mod_sum);
    // let mut a_mod_sum_sum = a.iter().fold(0, |a, b| (a + b * a_mod_sum) % MOD);
    // println!("{}", a_mod_sum_sum);

    let mut a_mod_sum_sum = a.iter().fold(0, |a, b| (a + b) % MOD).pow(2) % MOD;

    let diag_sum = a.iter().fold(0, |a, b| (a + b.pow(2)) % MOD);
    // println!("{}", diag_sum);
    
    if a_mod_sum_sum < diag_sum {
        a_mod_sum_sum += MOD;
    }

    let res;
    if (a_mod_sum_sum - diag_sum) % 2 == 0 {
        res = ((a_mod_sum_sum - diag_sum) / 2) % MOD;
    } else {
        res = ((a_mod_sum_sum + MOD - diag_sum) / 2) % MOD;
    }

    println!("{}", res);
}
