/// LeetCode #2926 - Maximum Balanced Subsequence Sum
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i64>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![i64::MIN / 4; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i64) {
        while x <= self.n {
            self.c[x] = self.c[x].max(v);
            x += x & (!x + 1);
        }
    }

    fn query(&self, mut x: usize) -> i64 {
        let mut mx = i64::MIN / 4;
        while x > 0 {
            mx = mx.max(self.c[x]);
            x -= x & (!x + 1);
        }
        mx
    }
}

fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
    let arr: Vec<i64> = nums
        .iter()
        .enumerate()
        .map(|(i, &x)| x as i64 - i as i64)
        .collect();
    let mut sorted = arr.clone();
    sorted.sort_unstable();
    sorted.dedup();

    let mut tree = BinaryIndexedTree::new(sorted.len());
    for (i, &x) in nums.iter().enumerate() {
        let j = sorted.binary_search(&(x as i64 - i as i64)).unwrap() + 1;
        let v = tree.query(j).max(0) + x as i64;
        tree.update(j, v);
    }
    tree.query(sorted.len())
}

fn main() {
    println!("{}", max_balanced_subsequence_sum(vec![3, 3, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_balanced_subsequence_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_balanced_subsequence_sum(vec![3, 3, 5, 6]), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_balanced_subsequence_sum(vec![5, -1, -3, 8]), 13);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_balanced_subsequence_sum(vec![-2, -1]), -1);
    }
}
