/// LeetCode #2767 - Partition String Into Minimum Beautiful Substrings
fn minimum_beautiful_substrings(s: &str) -> i32 {
    let n = s.len();
    let bytes = s.as_bytes();

    // precompute all binary reps of powers of 5 up to 2^15
    let mut powers: std::collections::HashSet<u32> = std::collections::HashSet::new();
    let mut p = 1u64;
    while p <= (1u64 << 15) {
        powers.insert(p as u32);
        p *= 5;
    }

    let inf = i32::MAX / 2;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;

    for i in 0..n {
        if bytes[i] == b'0' {
            continue;
        }
        let mut val = 0u32;
        for j in i..n {
            val = (val << 1) | (bytes[j] - b'0') as u32;
            if powers.contains(&val) && dp[i] != inf {
                dp[j + 1] = dp[j + 1].min(dp[i] + 1);
            }
        }
    }

    if dp[n] == inf { -1 } else { dp[n] }
}

fn main() {
    println!("{}", minimum_beautiful_substrings("1011"));
}

#[cfg(test)]
mod tests {
    use super::minimum_beautiful_substrings;

    #[test]
    fn example_one() {
        assert_eq!(minimum_beautiful_substrings("1011"), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_beautiful_substrings("111"), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_beautiful_substrings("0"), -1);
    }
}
