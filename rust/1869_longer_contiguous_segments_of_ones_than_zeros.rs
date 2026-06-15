/// LeetCode #1869 - Longer Contiguous Segments of Ones than Zeros
fn check_zero_ones(s: String) -> bool {
    let longest = |x: u8| -> usize {
        let mut cnt = 0usize;
        let mut mx = 0usize;
        for &c in s.as_bytes() {
            if c == x {
                cnt += 1;
                mx = mx.max(cnt);
            } else {
                cnt = 0;
            }
        }
        mx
    };
    longest(b'1') > longest(b'0')
}

fn main() {
    println!("{}", check_zero_ones("1101".into()));
}

#[cfg(test)]
mod tests {
    use super::check_zero_ones;

    #[test]
    fn example_one() {
        assert!(check_zero_ones("1101".into()));
    }
}
