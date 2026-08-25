/// LeetCode #3406 - Find the Lexicographically Largest String From the Box II
fn last_substring(s: &[u8]) -> usize {
    let mut i = 0;
    let mut j = 1;
    let mut k = 0;
    while j + k < s.len() {
        if s[i + k] == s[j + k] {
            k += 1;
        } else if s[i + k] < s[j + k] {
            i += k + 1;
            k = 0;
            if i >= j {
                j = i + 1;
            }
        } else {
            j += k + 1;
            k = 0;
        }
    }
    i
}

fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
        return word;
    }
    let n = word.len();
    let bytes = word.as_bytes();
    let i = last_substring(bytes);
    let take = n - num_friends as usize + 1;
    let end = (i + take).min(n);
    String::from_utf8(bytes[i..end].to_vec()).unwrap()
}

fn main() {
    println!("{}", answer_string("dbca".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::answer_string;

    #[test]
    fn example1() {
        assert_eq!(answer_string("dbca".into(), 2), "dbc");
    }

    #[test]
    fn example2() {
        assert_eq!(answer_string("gggg".into(), 4), "g");
    }
}
