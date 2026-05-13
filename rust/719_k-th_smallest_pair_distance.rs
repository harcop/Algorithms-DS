/// LeetCode #719 - Find K-th Smallest Pair Distance
fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();

    fn count_pairs(nums: &Vec<i32>, guess: i32) -> i32 {
        let mut count = 0;
        let mut left = 0usize;

        for right in 0..nums.len() {
            while nums[right] - nums[left] > guess {
                left += 1;
            }

            count += (right - left) as i32;
        }

        count
    }

    let mut low = 0;
    let mut high = nums[nums.len() - 1] - nums[0];

    while low < high {
        let mid = low + (high - low) / 2;

        if count_pairs(&nums, mid) >= k {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn main() {
    println!("{}", smallest_distance_pair(vec![1,3,1], 1));
}

#[cfg(test)]
mod tests {
    use super::smallest_distance_pair;

    #[test]
    fn example_one() {
        assert_eq!(smallest_distance_pair(vec![1,3,1], 1), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_distance_pair(vec![1,1,1], 2), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_distance_pair(vec![1,6,1], 3), 5);
    }
}