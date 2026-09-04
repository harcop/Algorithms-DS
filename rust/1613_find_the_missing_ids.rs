/// LeetCode #1613 - Find the Missing IDs (SQL; Rust analogue)
use std::collections::HashSet;

fn missing_ids(customers: Vec<(i32, String)>) -> Vec<i32> {
    if customers.is_empty() {
        return vec![];
    }
    let ids: HashSet<i32> = customers.iter().map(|(id, _)| *id).collect();
    let max = *ids.iter().max().unwrap();
    (1..=max).filter(|i| !ids.contains(i)).collect()
}

fn main() {
    println!("{:?}", missing_ids(vec![]));
}

#[cfg(test)]
mod tests {
    use super::missing_ids;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Alice".into()),
            (4, "Bob".into()),
            (7, "Alex".into()),
        ];
        assert_eq!(missing_ids(customers), vec![2, 3, 5, 6]);
    }
}
