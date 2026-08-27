/// LeetCode #3442 - Maximum Difference Between Even and Odd Frequency I
fn max_difference(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut a = 0;
    let mut b = i32::MAX;
    for &v in &cnt {
        if v == 0 {
            continue;
        }
        if v % 2 == 1 {
            a = a.max(v);
        } else {
            b = b.min(v);
        }
    }
    a - b
}

fn main() {
    println!("{}", max_difference("aaaaabbc".into()));
}

#[cfg(test)]
mod tests {
    use super::max_difference;

    #[test]
    fn example1() {
        assert_eq!(max_difference("aaaaabbc".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_difference("abcabcab".into()), 1);
    }
}
