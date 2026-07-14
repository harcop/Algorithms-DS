/// LeetCode #2390 - Removing Stars From a String
fn remove_stars(s: String) -> String {
    let mut ans = String::new();
    for c in s.chars() {
        if c == '*' {
            ans.pop();
        } else {
            ans.push(c);
        }
    }
    ans
}

fn main() {
    println!("{}", remove_stars("leet**cod*e".to_string()));
}

#[cfg(test)]
mod tests {
    use super::remove_stars;

    #[test]
    fn example_one() {
        assert_eq!(remove_stars("leet**cod*e".to_string()), "lecoe");
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_stars("erase*****".to_string()), "");
    }
}
