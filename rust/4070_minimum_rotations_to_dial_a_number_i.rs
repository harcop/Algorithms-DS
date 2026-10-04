/// LeetCode #4070 - Minimum Rotations to Dial a Number I
fn min_rotations(s: String) -> i32 {
    let mut pos = 0i32;
    let mut total = 0i32;
    for b in s.bytes() {
        let digit = (b - b'0') as i32;
        let diff = (digit - pos).abs();
        total += diff.min(10 - diff);
        pos = digit;
    }
    total
}

fn main() {
    println!("{}", min_rotations("0192837465".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_rotations;

    #[test]
    fn example1() {
        assert_eq!(min_rotations("0192837465".to_string()), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(min_rotations("1200210200".to_string()), 12);
    }
}
