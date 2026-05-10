/// LeetCode #611 - Valid Triangle Number
fn triangle_number(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = 0i32;
    for k in (2..n).rev() {
        let mut i = 0usize;
        let mut j = k - 1;
        while i < j {
            if nums[i] + nums[j] > nums[k] {
                ans += (j - i) as i32;
                j -= 1;
            } else {
                i += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", triangle_number(vec![2, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::triangle_number;

    #[test]
    fn example_one() {
        assert_eq!(triangle_number(vec![2, 2, 3, 4]), 3);
    }
}
