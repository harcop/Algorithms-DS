/// LeetCode #344 - Reverse String
fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

fn main() {
    let mut s = vec!['h','e','l','l','o'];
    reverse_string(&mut s);
    println!("{:?}", s);
}

#[cfg(test)]
mod tests {
    use super::reverse_string;

    #[test]
    fn example_one() {
        let mut s = vec!['h','e','l','l','o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o','l','l','e','h']);
    }
}
