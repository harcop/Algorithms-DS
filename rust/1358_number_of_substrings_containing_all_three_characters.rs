/// LeetCode #1358 - Number Of Substrings Containing All Three Characters

fn number_of_substrings_containing_all_three(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut count = [0i32; 3];
    let mut have = 0i32;
    let mut ans = 0i32;
    let mut left = 0usize;
    for right in 0..n {
        let idx = (bytes[right] - b'a') as usize;
        if count[idx] == 0 {
            have += 1;
        }
        count[idx] += 1;
        while have == 3 {
            ans += (n - right) as i32;
            let lidx = (bytes[left] - b'a') as usize;
            count[lidx] -= 1;
            if count[lidx] == 0 {
                have -= 1;
            }
            left += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_substrings_containing_all_three("abcabc".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_substrings_containing_all_three;

    #[test]
    fn example_one() {
        assert_eq!(number_of_substrings_containing_all_three("abcabc".into()), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_substrings_containing_all_three("aaacb".into()), 3);
    }
}
