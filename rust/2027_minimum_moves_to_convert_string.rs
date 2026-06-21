/// LeetCode #2027 - Minimum Moves to Convert String
fn minimum_moves(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ans = 0;
    let mut i = 0usize;
    while i < s.len() {
        if s[i] == b'X' {
            ans += 1;
            i += 3;
        } else {
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_moves("XXX".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_moves;

    #[test]
    fn example_one() {
        assert_eq!(minimum_moves("XXX".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_moves("XXOX".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_moves("OOOO".into()), 0);
    }
}
