/// LeetCode #3667 - Sort Array By Absolute Value (premium)
fn sort_by_absolute_value(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_by_key(|&x| x.abs());
    nums
}

fn main() {
    println!("{:?}", sort_by_absolute_value(vec![3, -1, -4, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::sort_by_absolute_value;

    #[test]
    fn example1() {
        let ans = sort_by_absolute_value(vec![3, -1, -4, 1, 5]);
        let abs: Vec<_> = ans.iter().map(|x| x.abs()).collect();
        assert!(abs.windows(2).all(|w| w[0] <= w[1]));
        let mut sorted = ans.clone();
        sorted.sort_unstable();
        let mut expected = vec![-4, -1, 1, 3, 5];
        expected.sort_unstable();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn example2() {
        let ans = sort_by_absolute_value(vec![-100, 100]);
        assert_eq!(ans.iter().map(|x| x.abs()).collect::<Vec<_>>(), vec![100, 100]);
    }
}
