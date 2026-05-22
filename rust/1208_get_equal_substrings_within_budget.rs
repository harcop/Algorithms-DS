/// LeetCode #1208 - Get Equal Substrings Within Budget
fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let b1 = s.as_bytes();
    let b2 = t.as_bytes();
    let n = b1.len();
    let mut l = 0usize;
    let mut sum = 0i32;
    let mut best = 0i32;
    for r in 0..n {
        sum += (b1[r] as i32 - b2[r] as i32).unsigned_abs() as i32;
        while sum > max_cost {
            sum -= (b1[l] as i32 - b2[l] as i32).unsigned_abs() as i32;
            l += 1;
        }
        best = best.max((r + 1 - l) as i32);
    }
    best
}

fn main() {
    println!("{}", equal_substring("abcd".into(), "bcdf".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::equal_substring;

    #[test]
    fn example_one() {
        assert_eq!(equal_substring("abcd".into(), "bcdf".into(), 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(equal_substring("abcd".into(), "cdef".into(), 3), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(equal_substring("abcd".into(), "acde".into(), 0), 1);
    }
}
