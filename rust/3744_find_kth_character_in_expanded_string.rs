/// LeetCode #3744 - Find Kth Character in Expanded String (premium)
fn kth_character(s: String, mut k: i64) -> String {
    for w in s.split(' ') {
        let m = (1 + w.len() as i64) * w.len() as i64 / 2;
        if k == m {
            return " ".into();
        }
        if k > m {
            k -= m + 1;
        } else {
            let mut cur = 0i64;
            for (i, ch) in w.chars().enumerate() {
                cur += i as i64 + 1;
                if k < cur {
                    return ch.to_string();
                }
            }
        }
    }
    " ".into()
}

fn main() {
    println!("{}", kth_character("hello world".into(), 0));
}

#[cfg(test)]
mod tests {
    use super::kth_character;

    #[test]
    fn example1() {
        assert_eq!(kth_character("hello world".into(), 0), "h");
    }

    #[test]
    fn example2() {
        assert_eq!(kth_character("hello world".into(), 15), " ");
    }
}
