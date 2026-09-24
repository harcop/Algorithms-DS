/// LeetCode #3920 - Maximize Fixed Points After Deletions
struct MaxBit {
    c: Vec<i32>,
}

impl MaxBit {
    fn new(n: usize) -> Self {
        Self { c: vec![0; n + 1] }
    }

    fn update(&mut self, mut x: usize, val: i32) {
        while x < self.c.len() {
            self.c[x] = self.c[x].max(val);
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut ans = 0;
        while x > 0 {
            ans = ans.max(self.c[x]);
            x -= x & x.wrapping_neg();
        }
        ans
    }
}

fn max_fixed_points(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut by_val: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, &v) in nums.iter().enumerate() {
        let v = v as usize;
        if v < n && i >= v {
            by_val[v].push(i - v);
        }
    }
    let mut bit = MaxBit::new(n);
    let mut ans = 0i32;
    for slacks in by_val {
        if slacks.is_empty() {
            continue;
        }
        let updates: Vec<(usize, i32)> = slacks
            .iter()
            .map(|&s| (s, 1 + bit.query(s + 1)))
            .collect();
        for (s, len) in updates {
            ans = ans.max(len);
            bit.update(s + 1, len);
        }
    }
    ans
}

fn main() {
    println!("{}", max_fixed_points(vec![0, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_fixed_points;

    #[test]
    fn example1() {
        assert_eq!(max_fixed_points(vec![0, 2, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_fixed_points(vec![3, 1, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(max_fixed_points(vec![1, 0, 1, 2]), 3);
    }
}
