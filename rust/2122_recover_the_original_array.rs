/// LeetCode #2122 - Recover the Original Array
use std::collections::HashMap;

fn recover_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut counts = HashMap::new();
    for &num in &nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    for i in 1..nums.len() {
        let diff = nums[i] - nums[0];
        if diff <= 0 || diff % 2 == 1 {
            continue;
        }

        let mut counts = counts.clone();
        let mut ans = Vec::with_capacity(nums.len() / 2);
        let mut ok = true;

        for &num in &nums {
            if counts.get(&num).copied().unwrap_or(0) == 0 {
                continue;
            }
            let high = num + diff;
            if counts.get(&high).copied().unwrap_or(0) == 0 {
                ok = false;
                break;
            }

            *counts.get_mut(&num).unwrap() -= 1;
            *counts.get_mut(&high).unwrap() -= 1;
            ans.push(num + diff / 2);
        }

        if ok && ans.len() == nums.len() / 2 {
            return ans;
        }
    }

    Vec::new()
}

fn main() {
    println!("{:?}", recover_array(vec![2, 10, 6, 4, 8, 12]));
}

#[cfg(test)]
mod tests {
    use super::recover_array;

    #[test]
    fn example_one() {
        assert_eq!(recover_array(vec![2, 10, 6, 4, 8, 12]), vec![3, 7, 11]);
    }

    #[test]
    fn example_two() {
        assert_eq!(recover_array(vec![1, 1, 3, 3]), vec![2, 2]);
    }

    #[test]
    fn example_three() {
        assert_eq!(recover_array(vec![5, 435]), vec![220]);
    }
}
