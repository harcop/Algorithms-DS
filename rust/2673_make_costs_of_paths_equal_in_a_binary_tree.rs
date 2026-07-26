/// LeetCode #2673 - Make Costs of Paths Equal in a Binary Tree
fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = n >> 1;
    while i > 0 {
        let l = (i << 1) as usize;
        let r = ((i << 1) | 1) as usize;
        ans += (cost[l - 1] - cost[r - 1]).abs();
        cost[i as usize - 1] += cost[l - 1].max(cost[r - 1]);
        i -= 1;
    }
    ans
}

fn main() {
    println!("{}", min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_increments;

    #[test]
    fn example_one() {
        assert_eq!(min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_increments(3, vec![5, 3, 3]), 0);
    }
}
