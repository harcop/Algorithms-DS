/// LeetCode #1100 - Find K-Length Substrings With No Repeated Characters
fn num_k_len_substr_with_no_repeats(s: String, k: i32) -> i32 {
    let k = k as usize;
    if k == 0 || s.len() < k {
        return 0;
    }
    let b = s.as_bytes();
    let mut cnt = [0i32; 256];
    let mut distinct = 0i32;
    let mut left = 0usize;
    let mut ans = 0i32;
    for right in 0..b.len() {
        let c = b[right] as usize;
        cnt[c] += 1;
        if cnt[c] == 1 {
            distinct += 1;
        }
        while distinct > k as i32 || right + 1 - left > k {
            let out = b[left] as usize;
            cnt[out] -= 1;
            if cnt[out] == 0 {
                distinct -= 1;
            }
            left += 1;
        }
        if distinct == k as i32 && right + 1 - left == k {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", num_k_len_substr_with_no_repeats("havefunonleetcode".into(), 5));
}

#[cfg(test)]
mod tests {
    use super::num_k_len_substr_with_no_repeats;

    #[test]
    fn example_one() {
        assert_eq!(num_k_len_substr_with_no_repeats("havefunonleetcode".into(), 5), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_k_len_substr_with_no_repeats("home".into(), 5), 0);
    }
}
