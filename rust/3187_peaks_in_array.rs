/// LeetCode #3187 - Peaks in Array
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, delta: i32) {
        while x <= self.n {
            self.c[x] += delta;
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x &= x - 1;
        }
        s
    }
}

fn is_peak(nums: &[i32], i: usize) -> bool {
    i > 0 && i + 1 < nums.len() && nums[i - 1] < nums[i] && nums[i] > nums[i + 1]
}

fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let mut tree = BinaryIndexedTree::new(n.saturating_sub(1).max(1));
    for i in 1..n.saturating_sub(1) {
        if is_peak(&nums, i) {
            tree.update(i, 1);
        }
    }
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let l = (q[1] + 1).max(1) as usize;
            let r = (q[2] - 1).max(0) as usize;
            if l > r {
                ans.push(0);
            } else {
                ans.push(tree.query(r) - tree.query(l - 1));
            }
        } else {
            let idx = q[1] as usize;
            let val = q[2];
            let left = idx.saturating_sub(1);
            let right = (idx + 1).min(n - 1);
            for i in left..=right {
                if is_peak(&nums, i) {
                    tree.update(i, -1);
                }
            }
            nums[idx] = val;
            for i in left..=right {
                if is_peak(&nums, i) {
                    tree.update(i, 1);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_of_peaks(vec![3, 1, 4, 2, 5], vec![vec![2, 3, 4], vec![1, 0, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_of_peaks;

    #[test]
    fn example1() {
        assert_eq!(
            count_of_peaks(vec![3, 1, 4, 2, 5], vec![vec![2, 3, 4], vec![1, 0, 4]]),
            vec![0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_of_peaks(
                vec![4, 1, 4, 2, 1, 5],
                vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]]
            ),
            vec![0, 1]
        );
    }
}
