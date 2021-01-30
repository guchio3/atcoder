/// 1 start なことに気をつける
struct BIT {
    n: usize,
    nodes: Vec<i64>,
    num_of_inversions: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT {
            n: n,
            nodes: vec![0; n + 1],
            num_of_inversions: vec![0; n],
        }
    }

    fn add(&mut self, mut i: usize, value: i64) {
        while i <= self.n {
            self.nodes[i] += value;
            i += (i as i64 & -(i as i64)) as usize;
        }
    }

    fn range_sum_from_start_to(&self, mut to: usize) -> i64 {
        let mut res = 0;
        while to > 0 {
            res += self.nodes[to];
            to -= (to as i64 & -(to as i64)) as usize;
        }
        res
    }
    fn range_sum_from_to(&self, from: usize, to: usize) -> i64 {
        self.range_sum_from_start_to(to) - self.range_sum_from_start_to(from)
    }

    fn calc_num_of_inversions(&mut self, nums: &Vec<i64>) -> i64 {
        let mut res = 0;
        for i in 0..nums.len() {
            let num = nums[i] as usize;
            let num_of_inversions = i as i64 - self.range_sum_from_start_to(num);
            self.num_of_inversions.push(num_of_inversions);
            res += num_of_inversions;
            self.add(num, 1);
        }
        res
    }
}
