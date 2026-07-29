/// LeetCode #2756 - Query Batching (JS problem; Rust analogue)
/// Models a synchronous batch query: given a list of keys and a resolver function,
/// batch all keys and return the results in order.
fn query_multiple<F>(keys: &[&str], resolver: F) -> Vec<String>
where
    F: Fn(&str) -> String,
{
    keys.iter().map(|k| resolver(k)).collect()
}

fn main() {
    let results = query_multiple(&["a", "b", "c"], |k| format!("{}!", k));
    println!("{:?}", results);
}

#[cfg(test)]
mod tests {
    use super::query_multiple;

    #[test]
    fn example_one() {
        let results = query_multiple(&["a", "b", "c"], |k| format!("{}!", k));
        assert_eq!(results, vec!["a!", "b!", "c!"]);
    }

    #[test]
    fn example_empty() {
        let results = query_multiple(&[], |k| k.to_string());
        assert!(results.is_empty());
    }
}
