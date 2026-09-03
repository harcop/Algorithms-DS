/// LeetCode #3557 - Find Maximum Number of Non Intersecting Substrings
fn max_substrings(word: String) -> i32 {
    let mut ans = 0;
    let mut first = [None; 26];
    for (i, c) in word.bytes().enumerate() {
        let idx = (c - b'a') as usize;
        if first[idx].is_none() {
            first[idx] = Some(i);
        } else if i - first[idx].unwrap() + 1 >= 4 {
            ans += 1;
            first = [None; 26];
        }
    }
    ans
}

fn main() {
    println!("{}", max_substrings("abcdeafdef".into()));
}

#[cfg(test)]
mod tests {
    use super::max_substrings;

    #[test]
    fn example1() {
        assert_eq!(max_substrings("abcdeafdef".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_substrings("bcdaaaab".into()), 1);
    }
}
