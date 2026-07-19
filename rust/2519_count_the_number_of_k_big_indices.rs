/// LeetCode #2519 - Count the Number of K-Big Indices
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

fn k_big_indices(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut tree1 = BinaryIndexedTree::new(n);
    let mut tree2 = BinaryIndexedTree::new(n);
    for &v in &nums {
        tree2.update(v as usize, 1);
    }
    let mut ans = 0;
    for &v in &nums {
        let v = v as usize;
        tree2.update(v, -1);
        if tree1.query(v - 1) >= k && tree2.query(v - 1) >= k {
            ans += 1;
        }
        tree1.update(v, 1);
    }
    ans
}

fn main() {
    println!("{}", k_big_indices(vec![2, 3, 6, 5, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::k_big_indices;

    #[test]
    fn example_one() {
        assert_eq!(k_big_indices(vec![2, 3, 6, 5, 2, 3], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_big_indices(vec![1, 1, 1], 3), 0);
    }
}
