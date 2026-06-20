/// LeetCode #1980 - Find Unique Binary String
fn find_different_binary_string(nums: Vec<String>) -> String {
    let mut mask = 0usize;
    for x in &nums {
        mask |= 1 << x.bytes().filter(|&b| b == b'1').count();
    }
    for i in 0..=nums.len() {
        if (mask >> i) & 1 == 0 {
            return format!("{}{}", "1".repeat(i), "0".repeat(nums.len() - i));
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", find_different_binary_string(vec!["01".into(), "10".into()]));
}

#[cfg(test)]
mod tests {
    use super::find_different_binary_string;

    #[test]
    fn example_one() {
        let ans = find_different_binary_string(vec!["01".into(), "10".into()]);
        assert!(ans == "11" || ans == "00");
    }

    #[test]
    fn example_two() {
        let ans = find_different_binary_string(vec!["00".into()]);
        assert_eq!(ans, "1");
    }
}
