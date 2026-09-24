/// LeetCode #3907 - Count Smaller Elements With Opposite Parity
struct Bit {
    c: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self { c: vec![0; n + 1] }
    }

    fn update(&mut self, mut x: usize, delta: i32) {
        while x < self.c.len() {
            self.c[x] += delta;
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn count_smaller_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let rank = |x: i32| -> usize { sorted.binary_search(&x).unwrap() + 1 };
    let m = sorted.len();
    let mut bits = [Bit::new(m), Bit::new(m)];
    let mut ans = vec![0i32; n];
    for i in (0..n).rev() {
        let x = rank(nums[i]);
        let parity = (nums[i] & 1) as usize;
        ans[i] = bits[parity ^ 1].query(x - 1);
        bits[parity].update(x, 1);
    }
    ans
}

fn main() {
    println!("{:?}", count_smaller_opposite_parity(vec![5, 2, 4, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_smaller_opposite_parity;

    #[test]
    fn example1() {
        assert_eq!(
            count_smaller_opposite_parity(vec![5, 2, 4, 1, 3]),
            vec![2, 1, 2, 0, 0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(count_smaller_opposite_parity(vec![4, 4, 1]), vec![1, 1, 0]);
    }

    #[test]
    fn example3() {
        assert_eq!(count_smaller_opposite_parity(vec![7]), vec![0]);
    }
}
