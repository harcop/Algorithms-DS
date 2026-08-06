/// LeetCode #3036 - Number of Subarrays That Match a Pattern II
fn compare(a: i32, b: i32) -> i32 {
    if a == b {
        0
    } else if a < b {
        1
    } else {
        -1
    }
}

fn build_lps(pat: &[i32]) -> Vec<usize> {
    let mut lps = vec![0usize; pat.len()];
    let mut len = 0usize;
    for i in 1..pat.len() {
        while len > 0 && pat[i] != pat[len] {
            len = lps[len - 1];
        }
        if pat[i] == pat[len] {
            len += 1;
        }
        lps[i] = len;
    }
    lps
}

fn kmp_count(text: &[i32], pat: &[i32]) -> usize {
    if pat.is_empty() {
        return text.len() + 1;
    }
    if text.len() < pat.len() {
        return 0;
    }
    let lps = build_lps(pat);
    let mut j = 0usize;
    let mut count = 0usize;
    for &c in text {
        while j > 0 && c != pat[j] {
            j = lps[j - 1];
        }
        if c == pat[j] {
            j += 1;
        }
        if j == pat.len() {
            count += 1;
            j = lps[j - 1];
        }
    }
    count
}

fn to_diff_pattern(nums: &[i32]) -> Vec<i32> {
    nums.windows(2)
        .map(|w| compare(w[0], w[1]))
        .collect()
}

fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let s = to_diff_pattern(&nums);
    kmp_count(&s, &pattern) as i32
}

fn main() {
    println!("{}", count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_matching_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
            2
        );
    }
}
