/// LeetCode #3723 - Maximize Sum of Squares of Digits
fn max_sum_of_squares(num: i32, sum: i32) -> String {
    if (num as i64) * 9 < sum as i64 {
        return String::new();
    }
    let k = (sum / 9) as usize;
    let s = sum % 9;
    let mut ans = vec![b'9'; k];
    if s > 0 {
        ans.push(b'0' + s as u8);
    }
    ans.resize(num as usize, b'0');
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", max_sum_of_squares(2, 3));
}

#[cfg(test)]
mod tests {
    use super::max_sum_of_squares;

    #[test]
    fn example1() {
        assert_eq!(max_sum_of_squares(2, 3), "30");
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum_of_squares(2, 17), "98");
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum_of_squares(1, 10), "");
    }
}
