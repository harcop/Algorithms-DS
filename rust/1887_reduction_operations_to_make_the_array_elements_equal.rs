/// LeetCode #1887 - Reduction Operations to Make the Array Elements Equal
fn reduction_operations(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut ans = 0i32;
    let mut cnt = 0i32;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            cnt += 1;
        }
        ans += cnt;
    }
    ans
}

fn main() {
    println!("{}", reduction_operations(vec![5, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::reduction_operations;

    #[test]
    fn example_one() {
        assert_eq!(reduction_operations(vec![5, 1, 3]), 3);
    }
}
