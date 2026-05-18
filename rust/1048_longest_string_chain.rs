/// LeetCode #1048 - Longest String Chain
fn longest_str_chain(words: Vec<String>) -> i32 {
    use std::collections::HashMap;
    let mut words = words;
    words.sort_by_key(|w| w.len());
    let mut dp: HashMap<String, i32> = HashMap::new();
    let mut ans = 0i32;
    for w in words {
        let mut best = 1;
        for i in 0..w.len() {
            let mut key: Vec<char> = w.chars().collect();
            key.remove(i);
            let prev: String = key.into_iter().collect();
            if let Some(&v) = dp.get(&prev) {
                best = best.max(v + 1);
            }
        }
        dp.insert(w.clone(), best);
        ans = ans.max(best);
    }
    ans
}

fn main() {
    println!("{}", longest_str_chain(vec!["a".into(), "b".into(), "ba".into(), "bca".into(), "bda".into(), "bdca".into()]));
}

#[cfg(test)]
mod tests {
    use super::longest_str_chain;

    #[test]
    fn example_one() {
        assert_eq!(longest_str_chain(vec!["a".into(), "b".into(), "ba".into(), "bca".into(), "bda".into(), "bdca".into()]), 4);
    }
}
