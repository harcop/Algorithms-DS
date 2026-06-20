/// LeetCode #1985 - Find the Kth Largest Integer in the Array
fn cmp_desc(a: &String, b: &String) -> std::cmp::Ordering {
    if a.len() != b.len() {
        b.len().cmp(&a.len())
    } else {
        b.as_str().cmp(a.as_str())
    }
}

fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
    let mut nums = nums;
    nums.sort_by(cmp_desc);
    nums[k as usize - 1].clone()
}

fn main() {
    println!(
        "{}",
        kth_largest_number(vec!["3".into(), "6".into(), "7".into(), "10".into()], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::kth_largest_number;

    #[test]
    fn example_one() {
        assert_eq!(
            kth_largest_number(vec!["3".into(), "6".into(), "7".into(), "10".into()], 4),
            "3"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            kth_largest_number(vec!["2".into(), "21".into(), "12".into(), "1".into()], 3),
            "2"
        );
    }
}
