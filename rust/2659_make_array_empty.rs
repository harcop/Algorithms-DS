/// LeetCode #2659 - Make Array Empty
use std::collections::HashMap;

struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
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
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn count_operations_to_empty_array(mut nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut pos = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        pos.insert(x, i);
    }
    nums.sort_unstable();
    let mut tree = BinaryIndexedTree::new(n);
    let mut ans = pos[&nums[0]] as i64 + 1;
    for k in 0..n - 1 {
        let i = pos[&nums[k]];
        let j = pos[&nums[k + 1]];
        let mut d = j as i64 - i as i64 - (tree.query(j + 1) - tree.query(i + 1)) as i64;
        if i > j {
            d += (n - k) as i64;
        }
        ans += d;
        tree.update(i + 1, 1);
    }
    ans
}

fn main() {
    println!("{}", count_operations_to_empty_array(vec![3, 4, -1]));
}

#[cfg(test)]
mod tests {
    use super::count_operations_to_empty_array;

    #[test]
    fn example_one() {
        assert_eq!(count_operations_to_empty_array(vec![3, 4, -1]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_operations_to_empty_array(vec![1, 2, 4, 3]), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_operations_to_empty_array(vec![1, 2, 3]), 3);
    }
}
