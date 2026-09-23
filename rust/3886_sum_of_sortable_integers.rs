/// LeetCode #3886 - Sum of Sortable Integers
fn contains_subslice(hay: &[i32], needle: &[i32]) -> bool {
    if needle.is_empty() {
        return true;
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

fn is_k_sortable(nums: &[i32], sorted: &[i32], k: usize) -> bool {
    let n = nums.len();
    for b in (0..n).step_by(k) {
        let block = &nums[b..b + k];
        let target = &sorted[b..b + k];
        let mut doubled = Vec::with_capacity(2 * k);
        doubled.extend_from_slice(block);
        doubled.extend_from_slice(block);
        if !contains_subslice(&doubled, target) {
            return false;
        }
    }
    true
}

fn sum_sortable(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let mut ans = 0;
    for k in 1..=n {
        if n % k != 0 {
            continue;
        }
        if is_k_sortable(&nums, &sorted, k) {
            ans += k as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", sum_sortable(vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::sum_sortable;

    #[test]
    fn example1() {
        assert_eq!(sum_sortable(vec![3, 1, 2]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_sortable(vec![7, 6, 5]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_sortable(vec![5, 8]), 3);
    }
}
