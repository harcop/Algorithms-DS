/// LeetCode #162 - Find Peak Element
fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut lo = 0usize;
    let mut hi = nums.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] < nums[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", find_peak_element(vec![1, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_peak_element;

    fn is_peak(nums: &[i32], i: i32) -> bool {
        let i = i as usize;
        let v = nums[i];
        let l = nums.get(i.wrapping_sub(1)).copied();
        let r = nums.get(i + 1).copied();
        (l.is_none() || v > l.unwrap()) && (r.is_none() || v > r.unwrap())
    }

    #[test]
    fn example_one() {
        let nums = vec![1, 2, 3, 1];
        let i = find_peak_element(nums.clone());
        assert!(is_peak(&nums, i));
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let i = find_peak_element(nums.clone());
        assert!(is_peak(&nums, i));
    }
}
