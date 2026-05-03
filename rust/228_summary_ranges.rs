/// LeetCode #228 - Summary Ranges
fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut out = vec![];
    let n = nums.len();
    let mut i = 0usize;
    while i < n {
        let j = i;
        while i + 1 < n && nums[i + 1] == nums[i] + 1 {
            i += 1;
        }
        if j == i {
            out.push(nums[j].to_string());
        } else {
            out.push(format!("{}->{}", nums[j], nums[i]));
        }
        i += 1;
    }
    out
}

fn main() {
    println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::summary_ranges;

    #[test]
    fn example_one() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0", "2->4", "6", "8->9"]);
    }
}
