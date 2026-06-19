/// LeetCode #1968 - Array With Elements Not Equal to Average of Neighbors
fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    let m = (n + 1) / 2;
    let mut ans = Vec::with_capacity(n);
    for i in 0..m {
        ans.push(nums[i]);
        if i + m < n {
            ans.push(nums[i + m]);
        }
    }
    ans
}

fn main() {
    println!("{:?}", rearrange_array(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::rearrange_array;

    #[test]
    fn example_one() {
        assert_eq!(rearrange_array(vec![1, 2, 3, 4, 5]), vec![1, 4, 2, 5, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(rearrange_array(vec![6, 2, 0, 9, 7]), vec![0, 7, 2, 9, 6]);
    }
}
