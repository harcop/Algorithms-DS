/// LeetCode #3691 - Maximum Total Subarray Value II
use std::collections::BinaryHeap;

struct SparseTable {
    f_max: Vec<Vec<i32>>,
    f_min: Vec<Vec<i32>>,
    lg: Vec<usize>,
}

impl SparseTable {
    fn new(data: &[i32]) -> Self {
        let n = data.len();
        let max_log = if n == 0 {
            1
        } else {
            n.ilog2() as usize + 1
        };
        let mut f_max = vec![vec![0; max_log]; n];
        let mut f_min = vec![vec![0; max_log]; n];
        let mut lg = vec![0; n + 1];
        for i in 2..=n {
            lg[i] = lg[i >> 1] + 1;
        }
        for i in 0..n {
            f_max[i][0] = data[i];
            f_min[i][0] = data[i];
        }
        for j in 1..max_log {
            if (1usize << j) > n {
                break;
            }
            let lim = n + 1 - (1 << j);
            for i in 0..lim {
                f_max[i][j] = f_max[i][j - 1].max(f_max[i + (1 << (j - 1))][j - 1]);
                f_min[i][j] = f_min[i][j - 1].min(f_min[i + (1 << (j - 1))][j - 1]);
            }
        }
        Self { f_max, f_min, lg }
    }

    fn query_max(&self, l: usize, r: usize) -> i32 {
        let k = self.lg[r - l + 1];
        self.f_max[l][k].max(self.f_max[r + 1 - (1 << k)][k])
    }

    fn query_min(&self, l: usize, r: usize) -> i32 {
        let k = self.lg[r - l + 1];
        self.f_min[l][k].min(self.f_min[r + 1 - (1 << k)][k])
    }
}

fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let st = SparseTable::new(&nums);
    let mut pq = BinaryHeap::new();
    for l in 0..n {
        let val = st.query_max(l, n - 1) - st.query_min(l, n - 1);
        pq.push((val, l, n - 1));
    }
    let mut ans = 0i64;
    for _ in 0..k {
        let (val, l, r) = pq.pop().unwrap();
        ans += val as i64;
        if r > l {
            let nv = st.query_max(l, r - 1) - st.query_min(l, r - 1);
            pq.push((nv, l, r - 1));
        }
    }
    ans
}

fn main() {
    println!("{}", max_total_value(vec![1, 3, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_total_value;

    #[test]
    fn example1() {
        assert_eq!(max_total_value(vec![1, 3, 2], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}
