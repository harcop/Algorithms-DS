/// LeetCode #2416 - Sum of Prefix Scores of Strings
use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    count: i32,
    children: HashMap<u8, TrieNode>,
}

fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut root = TrieNode::default();

    for word in &words {
        let mut node = &mut root;
        for &ch in word.as_bytes() {
            node = node.children.entry(ch).or_default();
            node.count += 1;
        }
    }

    let mut ans = Vec::with_capacity(words.len());
    for word in &words {
        let mut node = &root;
        let mut total = 0;
        for &ch in word.as_bytes() {
            node = node.children.get(&ch).unwrap();
            total += node.count;
        }
        ans.push(total);
    }
    ans
}

fn main() {
    println!("{:?}", sum_prefix_scores(vec!["abc".to_string(), "ab".to_string(), "bc".to_string(), "b".to_string()]));
}

#[cfg(test)]
mod tests {
    use super::sum_prefix_scores;

    #[test]
    fn example_one() {
        assert_eq!(
            sum_prefix_scores(vec![
                "abc".to_string(),
                "ab".to_string(),
                "bc".to_string(),
                "b".to_string()
            ]),
            vec![5, 4, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sum_prefix_scores(vec!["abcd".to_string()]),
            vec![4]
        );
    }
}
