/// LeetCode #2030 - Smallest K-Length Subsequence With Occurrences of a Letter
fn smallest_subsequence(s: String, k: i32, letter: String, repetition: i32) -> String {
    let s = s.as_bytes();
    let letter = letter.as_bytes()[0];
    let k = k as usize;
    let mut required = repetition;
    let mut n_letters = s.iter().filter(|&&c| c == letter).count() as i32;
    let mut stack: Vec<u8> = Vec::new();

    for (i, &c) in s.iter().enumerate() {
        while !stack.is_empty()
            && stack.last().unwrap() > &c
            && stack.len() + s.len() - i - 1 >= k
            && (*stack.last().unwrap() != letter || n_letters > required)
        {
            if stack.pop().unwrap() == letter {
                required += 1;
            }
        }
        if stack.len() < k {
            if c == letter {
                stack.push(c);
                required -= 1;
            } else if (k - stack.len()) as i32 > required {
                stack.push(c);
            }
        }
        if c == letter {
            n_letters -= 1;
        }
    }
    String::from_utf8(stack).unwrap()
}

fn main() {
    println!(
        "{}",
        smallest_subsequence("leet".into(), 3, "e".into(), 1)
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_subsequence("leet".into(), 3, "e".into(), 1),
            "eet"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_subsequence("leetcode".into(), 4, "e".into(), 2),
            "ecde"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            smallest_subsequence("bb".into(), 2, "b".into(), 2),
            "bb"
        );
    }
}
