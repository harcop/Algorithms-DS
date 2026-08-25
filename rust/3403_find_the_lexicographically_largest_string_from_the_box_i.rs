/// LeetCode #3403 - Find the Lexicographically Largest String From the Box I
fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
        return word;
    }
    let n = word.len();
    let max_len = n - (num_friends as usize - 1);
    let bytes = word.as_bytes();
    let mut best: &[u8] = b"";
    for i in 0..n {
        let end = (i + max_len).min(n);
        if &bytes[i..end] > best {
            best = &bytes[i..end];
        }
    }
    String::from_utf8(best.to_vec()).unwrap()
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
