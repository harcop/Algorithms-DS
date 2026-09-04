/// LeetCode #528 - Random Pick with Weight
struct Solution {
    prefix: Vec<i32>,
    total: i32,
    rng: u64,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for x in w {
            sum += x;
            prefix.push(sum);
        }
        Solution {
            prefix,
            total: sum,
            rng: 0x9e3779b97f4a7c15,
        }
    }

    fn next_rand(&mut self) -> u64 {
        self.rng = self.rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.rng
    }

    fn pick_index(&mut self) -> i32 {
        let t = (self.next_rand() % self.total as u64) as i32 + 1;
        let mut lo = 0;
        let mut hi = self.prefix.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.prefix[mid] < t {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

fn main() {
    let mut s = Solution::new(vec![1]);
    println!("{}", s.pick_index());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let mut s = Solution::new(vec![1]);
        assert_eq!(s.pick_index(), 0);
    }

    #[test]
    fn example_two() {
        let mut s = Solution::new(vec![1, 3]);
        for _ in 0..50 {
            let i = s.pick_index();
            assert!(i == 0 || i == 1);
        }
    }
}
