/// LeetCode #1911 - Maximum Alternating Subsequence Sum
fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut f = 0i64;
    let mut g = 0i64;
    for &x in &nums {
        let x = x as i64;
        let nf = g - x;
        let ng = f + x;
        f = f.max(nf);
        g = g.max(ng);
    }
    f.max(g)
}

fn main() {
    println!("{}", max_alternating_sum(vec![4, 2, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_alternating_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_alternating_sum(vec![4, 2, 5, 3]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_alternating_sum(vec![1, 2, 3, 4, 5]), 5);
    }
}
