/// LeetCode #2950 - Number of Divisible Substrings
fn count_divisible_substrings(word: String) -> i32 {
    let groups = ["ab", "cde", "fgh", "ijk", "lmn", "opq", "rst", "uvw", "xyz"];
    let mut mp = [0i32; 26];
    for (i, g) in groups.iter().enumerate() {
        for c in g.bytes() {
            mp[(c - b'a') as usize] = (i + 1) as i32;
        }
    }
    let bytes = word.as_bytes();
    let n = bytes.len();
    let mut ans = 0;
    for i in 0..n {
        let mut s = 0;
        for j in i..n {
            s += mp[(bytes[j] - b'a') as usize];
            if s % (j - i + 1) as i32 == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_divisible_substrings("asdf".into()));
}

#[cfg(test)]
mod tests {
    use super::count_divisible_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_divisible_substrings("asdf".into()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_divisible_substrings("bdh".into()), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_divisible_substrings("abcd".into()), 6);
    }
}
