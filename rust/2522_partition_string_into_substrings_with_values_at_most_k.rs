/// LeetCode #2522 - Partition String Into Substrings With Values at Most K
fn minimum_partition(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let k = k as i64;
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut v = 0i64;
        let start = i;
        while i < n {
            let digit = (bytes[i] - b'0') as i64;
            if v * 10 + digit > k {
                break;
            }
            v = v * 10 + digit;
            i += 1;
        }
        if i == start {
            return -1;
        }
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", minimum_partition("165462".to_string(), 60));
}

#[cfg(test)]
mod tests {
    use super::minimum_partition;

    #[test]
    fn example_one() {
        assert_eq!(minimum_partition("165462".to_string(), 60), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_partition("238182".to_string(), 5), -1);
    }
}
