/// LeetCode #3597 - Partition String
use std::collections::HashSet;

fn partition_string(s: String) -> Vec<String> {
    let mut vis = HashSet::new();
    let mut ans = Vec::new();
    let mut t = String::new();
    for c in s.chars() {
        t.push(c);
        if vis.insert(t.clone()) {
            ans.push(t);
            t = String::new();
        }
    }
    ans
}

fn main() {
    println!("{:?}", partition_string("abbccccd".into()));
}

#[cfg(test)]
mod tests {
    use super::partition_string;

    #[test]
    fn example1() {
        assert_eq!(
            partition_string("abbccccd".into()),
            vec!["a", "b", "bc", "c", "cc", "d"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(partition_string("aaaa".into()), vec!["a", "aa"]);
    }
}
