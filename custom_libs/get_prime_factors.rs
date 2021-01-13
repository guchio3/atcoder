/// $BAG0x?tJ,2r$9$k4X?t(B
/// $BAG0x?t$,A4$FF~$C$?(B Vec $B$rJV$9(B
///
/// ex. 12 -> [2, 2, 3]
fn get_prime_factors(num: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut iter_num = num;
    for i in 2..=num {
        if i > iter_num {
            break;
        }
        while iter_num % i == 0 {
            res.push(i);
            iter_num /= i;
        }
    }
    res
}
