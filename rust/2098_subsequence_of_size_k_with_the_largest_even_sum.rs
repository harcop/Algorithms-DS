/// LeetCode #2098 - Subsequence of Size K With the Largest Even Sum
fn largest_even_sum(mut nums: Vec<i32>, k: i32) -> i64 {
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let k = k as usize;

    let selected = &nums[..k];
    let rest = &nums[k..];
    let sum: i64 = selected.iter().map(|&x| x as i64).sum();
    if sum % 2 == 0 {
        return sum;
    }

    let min_selected_odd = selected.iter().filter(|&&x| x % 2 != 0).min().copied();
    let min_selected_even = selected.iter().filter(|&&x| x % 2 == 0).min().copied();
    let max_rest_odd = rest.iter().filter(|&&x| x % 2 != 0).max().copied();
    let max_rest_even = rest.iter().filter(|&&x| x % 2 == 0).max().copied();

    let mut ans = -1i64;
    if let (Some(remove), Some(add)) = (min_selected_odd, max_rest_even) {
        ans = ans.max(sum - remove as i64 + add as i64);
    }
    if let (Some(remove), Some(add)) = (min_selected_even, max_rest_odd) {
        ans = ans.max(sum - remove as i64 + add as i64);
    }
    ans
}

fn main() {
    println!("{}", largest_even_sum(vec![4, 1, 5, 3, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::largest_even_sum;

    #[test]
    fn can_use_top_k_directly() {
        assert_eq!(largest_even_sum(vec![4, 1, 5, 3, 1], 3), 12);
    }

    #[test]
    fn swaps_to_make_even() {
        assert_eq!(largest_even_sum(vec![5, 3, 2, 2], 3), 10);
    }

    #[test]
    fn impossible() {
        assert_eq!(largest_even_sum(vec![1, 3, 5], 1), -1);
    }
}
