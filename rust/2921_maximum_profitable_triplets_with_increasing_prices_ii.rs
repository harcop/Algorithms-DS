/// LeetCode #2921 - Maximum Profitable Triplets With Increasing Prices II
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

    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            self.c[x] = self.c[x].max(v);
            x += x & (!x + 1);
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut mx = 0;
        while x > 0 {
            mx = mx.max(self.c[x]);
            x -= x & (!x + 1);
        }
        mx
    }
}

fn max_profit(prices: Vec<i32>, profits: Vec<i32>) -> i32 {
    let n = prices.len();
    let m = *prices.iter().max().unwrap() as usize;
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut tree1 = BinaryIndexedTree::new(m + 1);
    let mut tree2 = BinaryIndexedTree::new(m + 1);

    for i in 0..n {
        let x = prices[i] as usize;
        left[i] = tree1.query(x.saturating_sub(1));
        tree1.update(x, profits[i]);
    }
    for i in (0..n).rev() {
        let x = m + 1 - prices[i] as usize;
        right[i] = tree2.query(x.saturating_sub(1));
        tree2.update(x, profits[i]);
    }

    let mut ans = -1;
    for i in 0..n {
        if left[i] > 0 && right[i] > 0 {
            ans = ans.max(left[i] + profits[i] + right[i]);
        }
    }
    ans
}

fn main() {
    println!("{}", max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example_one() {
        assert_eq!(max_profit(vec![10, 2, 3, 4], vec![100, 2, 7, 10]), 19);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5], vec![1, 5, 3, 4, 6]), 15);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_profit(vec![4, 3, 2, 1], vec![33, 20, 19, 87]), -1);
    }
}
