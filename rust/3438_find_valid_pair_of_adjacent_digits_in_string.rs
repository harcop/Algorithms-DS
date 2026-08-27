/// LeetCode #3438 - Find Valid Pair of Adjacent Digits in String
fn find_valid_pair(s: String) -> String {
    let mut cnt = [0i32; 10];
    for &c in s.as_bytes() {
        cnt[(c - b'0') as usize] += 1;
    }
    let b = s.as_bytes();
    for w in b.windows(2) {
        let x = (w[0] - b'0') as i32;
        let y = (w[1] - b'0') as i32;
        if x != y && cnt[x as usize] == x && cnt[y as usize] == y {
            return format!("{x}{y}");
        }
    }
    String::new()
}

fn main() {
    println!("{}", find_valid_pair("2523533".into()));
}

#[cfg(test)]
mod tests {
    use super::find_valid_pair;

    #[test]
    fn example1() {
        assert_eq!(find_valid_pair("2523533".into()), "23");
    }

    #[test]
    fn example2() {
        assert_eq!(find_valid_pair("221".into()), "21");
    }

    #[test]
    fn example3() {
        assert_eq!(find_valid_pair("22".into()), "");
    }
}
