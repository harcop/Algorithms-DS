/// LeetCode #1023 - Camelcase Matching
fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    let p: Vec<char> = pattern.chars().collect();
    queries
        .into_iter()
        .map(|q| {
            let q: Vec<char> = q.chars().collect();
            let mut j = 0usize;
            for &c in &q {
                if j < p.len() && c == p[j] {
                    j += 1;
                } else if c.is_uppercase() {
                    return false;
                }
            }
            j == p.len()
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        camel_match(vec!["FooBar".into(), "FooBarTest".into()], "FB".into())
    );
}

#[cfg(test)]
mod tests {
    use super::camel_match;

    #[test]
    fn example_one() {
        assert_eq!(
            camel_match(vec!["FooBar".into(), "FooBarTest".into()], "FB".into()),
            vec![true, false]
        );
    }
}
