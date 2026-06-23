/// LeetCode #2067 - Number of Equal Count Substrings
use std::collections::HashMap;

fn equal_count_substrings(s: String, count: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let count = count as i32;
    let mut ans = 0i32;

    for i in 1..=26 {
        let k = i * count as usize;
        if k > n {
            break;
        }
        let mut cnt: HashMap<u8, i32> = HashMap::new();
        let mut t = 0i32;
        for j in 0..n {
            let c = s[j];
            let entry = cnt.entry(c).or_insert(0);
            *entry += 1;
            if *entry == count {
                t += 1;
            }
            if *entry == count + 1 {
                t -= 1;
            }
            if j >= k {
                let left = s[j - k];
                let entry = cnt.get_mut(&left).unwrap();
                *entry -= 1;
                if *entry == count {
                    t += 1;
                }
                if *entry == count - 1 {
                    t -= 1;
                }
            }
            if i as i32 == t {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", equal_count_substrings("aaabcbbcc".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::equal_count_substrings;

    #[test]
    fn example_one() {
        assert_eq!(equal_count_substrings("aaabcbbcc".into(), 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(equal_count_substrings("abcd".into(), 2), 0);
    }
}
