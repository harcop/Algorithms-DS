/// LeetCode #2167 - Minimum Time to Remove All Cars Containing Illegal Goods
fn minimum_time(s: String) -> i32 {
    let n = s.len() as i32;
    let mut left = 0i32;
    let mut ans = n;

    for (i, c) in s.bytes().enumerate() {
        if c == b'1' {
            left = (left + 2).min(i as i32 + 1);
        }
        ans = ans.min(left + n - i as i32 - 1);
    }

    ans
}

fn main() {
    println!("{}", minimum_time("1100101".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time("1100101".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_time("0010".into()), 2);
    }
}
