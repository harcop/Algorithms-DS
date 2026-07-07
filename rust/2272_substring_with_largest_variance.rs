/// LeetCode #2272 - Substring With Largest Variance
fn largest_variance(s: String) -> i32 {
    let mut ans = 0;
    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            if a != b {
                ans = ans.max(kadane(&s, a as char, b as char));
            }
        }
    }
    ans
}

fn kadane(s: &str, a: char, b: char) -> i32 {
    let mut ans = 0;
    let mut count_a = 0;
    let mut count_b = 0;
    let mut can_extend_prev_b = false;

    for c in s.chars() {
        if c != a && c != b {
            continue;
        }
        if c == a {
            count_a += 1;
        } else {
            count_b += 1;
        }
        if count_b > 0 {
            ans = ans.max(count_a - count_b);
        } else if count_b == 0 && can_extend_prev_b {
            ans = ans.max(count_a - 1);
        }
        if count_b > count_a {
            count_a = 0;
            count_b = 0;
            can_extend_prev_b = true;
        }
    }

    ans
}

fn main() {
    println!("{}", largest_variance("aababbb".to_string()));
}

#[cfg(test)]
mod tests {
    use super::largest_variance;

    #[test]
    fn example_one() {
        assert_eq!(largest_variance("aababbb".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_variance("abcde".to_string()), 0);
    }
}
