/// LeetCode #1794 - Count Pairs of Equal Substrings With Minimum Difference
use std::collections::HashMap;

fn count_quadruples(first_string: String, second_string: String) -> i32 {
    let last: HashMap<u8, usize> = second_string
        .bytes()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    let mut ans = 0i32;
    let mut mi = i32::MAX;
    for (i, c) in first_string.bytes().enumerate() {
        if let Some(&j) = last.get(&c) {
            let t = i as i32 - j as i32;
            if mi > t {
                mi = t;
                ans = 1;
            } else if mi == t {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_quadruples("abcd".into(), "bccda".into())
    );
}

#[cfg(test)]
mod tests {
    use super::count_quadruples;

    #[test]
    fn example_one() {
        assert_eq!(count_quadruples("abcd".into(), "bccda".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_quadruples("ab".into(), "cd".into()), 0);
    }
}
