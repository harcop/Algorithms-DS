/// LeetCode #3499 - Maximize Active Section with Trade I
fn max_active_sections_after_trade(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0i32;
    let mut pre = i32::MIN;
    let mut mx = 0i32;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && b[j] == b[i] {
            j += 1;
        }
        let cur = (j - i) as i32;
        if b[i] == b'1' {
            ans += cur;
        } else {
            mx = mx.max(pre.saturating_add(cur));
            pre = cur;
        }
        i = j;
    }
    ans + mx
}

fn main() {
    println!("{}", max_active_sections_after_trade("01".into()));
}

#[cfg(test)]
mod tests {
    use super::max_active_sections_after_trade;

    #[test]
    fn example1() {
        assert_eq!(max_active_sections_after_trade("01".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(max_active_sections_after_trade("0100".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(max_active_sections_after_trade("1000100".into()), 7);
    }

    #[test]
    fn example4() {
        assert_eq!(max_active_sections_after_trade("01010".into()), 4);
    }
}
