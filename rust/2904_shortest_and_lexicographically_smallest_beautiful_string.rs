/// LeetCode #2904 - Shortest and Lexicographically Smallest Beautiful String
fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let n = s.len();
    let bytes = s.as_bytes();
    let k = k as usize;
    let mut ans = String::new();

    for i in 0..n {
        let mut ones = 0;
        for j in i..n {
            if bytes[j] == b'1' {
                ones += 1;
            }
            if ones == k {
                let candidate = &s[i..=j];
                if ans.is_empty()
                    || candidate.len() < ans.len()
                    || (candidate.len() == ans.len() && candidate < ans.as_str())
                {
                    ans = candidate.to_string();
                }
                break;
            }
            if ones > k {
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", shortest_beautiful_substring("100011001".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::shortest_beautiful_substring;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_beautiful_substring("100011001".into(), 3),
            "11001"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_beautiful_substring("1011".into(), 2), "11");
    }

    #[test]
    fn example_three() {
        assert_eq!(shortest_beautiful_substring("000".into(), 1), "");
    }
}
