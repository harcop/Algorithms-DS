/// LeetCode #2733 - Neither Minimum nor Maximum
fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    let mut mi = 100;
    let mut mx = 0;
    for &x in &nums {
        mi = mi.min(x);
        mx = mx.max(x);
    }
    for &x in &nums {
        if x != mi && x != mx {
            return x;
        }
    }
    -1
}

fn main() {
    println!("{}", find_non_min_or_max(vec![3, 2, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_non_min_or_max;

    #[test]
    fn example_one() {
        let nums = vec![3, 2, 1, 4];
        let result = find_non_min_or_max(nums.clone());
        let mi = *nums.iter().min().unwrap();
        let mx = *nums.iter().max().unwrap();
        assert!(result != mi && result != mx);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_non_min_or_max(vec![1, 2]), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_non_min_or_max(vec![2, 1, 3]), 2);
    }
}
