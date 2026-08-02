/// LeetCode #2874 - Maximum Value of an Ordered Triplet II
fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut mx = 0;
    let mut mx_diff = 0;

    for x in nums {
        ans = ans.max(mx_diff as i64 * x as i64);
        mx_diff = mx_diff.max(mx - x);
        mx = mx.max(x);
    }
    ans
}

fn main() {
    println!("{}", maximum_triplet_value(vec![12, 6, 1, 2, 7]));
}

#[cfg(test)]
mod tests {
    use super::maximum_triplet_value;

    #[test]
    fn example_one() {
        assert_eq!(maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_triplet_value(vec![1, 2, 3]), 0);
    }
}
