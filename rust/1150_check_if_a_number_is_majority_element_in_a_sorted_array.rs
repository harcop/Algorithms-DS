/// LeetCode #1150 - Check If a Number Is Majority Element in a Sorted Array
fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    let n = nums.len();
    let need = n / 2 + 1;
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if nums[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo >= n || nums[lo] != target {
        return false;
    }
    let start = lo;
    lo = 0;
    hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if nums[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo - start >= need
}

fn main() {
    println!("{}", is_majority_element(vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5));
}

#[cfg(test)]
mod tests {
    use super::is_majority_element;

    #[test]
    fn example_one() {
        assert!(is_majority_element(vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5));
    }

    #[test]
    fn example_two() {
        assert!(!is_majority_element(vec![10, 100, 101, 101], 101));
    }
}
