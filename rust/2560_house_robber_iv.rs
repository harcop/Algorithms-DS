/// LeetCode #2560 - House Robber IV
fn num_stolen_houses(nums: &[i32], capacity: i32) -> i32 {
    let mut stolen = 0;
    let mut i = 0;
    while i < nums.len() {
        if nums[i] <= capacity {
            stolen += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    stolen
}

fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut l = *nums.iter().min().unwrap();
    let mut r = *nums.iter().max().unwrap();
    while l < r {
        let m = l + (r - l) / 2;
        if num_stolen_houses(&nums, m) >= k {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn main() {
    println!("{}", min_capability(vec![2, 3, 5, 9], 2));
}

#[cfg(test)]
mod tests {
    use super::min_capability;

    #[test]
    fn example_one() {
        assert_eq!(min_capability(vec![2, 3, 5, 9], 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_capability(vec![2, 7, 9, 3, 1], 2), 2);
    }
}
