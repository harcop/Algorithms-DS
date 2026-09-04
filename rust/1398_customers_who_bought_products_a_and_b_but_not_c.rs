/// LeetCode #1398 - Customers Who Bought Products A and B but Not C (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn bought_a_b_not_c(
    customers: Vec<(i32, String)>,
    orders: Vec<(i32, i32, String)>,
) -> Vec<(i32, String)> {
    let mut bought: HashMap<i32, HashSet<String>> = HashMap::new();
    for (_, cid, product) in orders {
        bought.entry(cid).or_default().insert(product);
    }
    let mut ans = Vec::new();
    for (id, name) in customers {
        if let Some(set) = bought.get(&id) {
            if set.contains("A") && set.contains("B") && !set.contains("C") {
                ans.push((id, name));
            }
        }
    }
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", bought_a_b_not_c(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::bought_a_b_not_c;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Daniel".into()),
            (2, "Diana".into()),
            (3, "Elizabeth".into()),
            (4, "Jhon".into()),
        ];
        let orders = vec![
            (10, 1, "A".into()),
            (20, 1, "B".into()),
            (30, 1, "D".into()),
            (40, 1, "C".into()),
            (50, 2, "A".into()),
            (60, 3, "A".into()),
            (70, 3, "B".into()),
            (80, 3, "D".into()),
            (90, 4, "C".into()),
        ];
        assert_eq!(
            bought_a_b_not_c(customers, orders),
            vec![(3, "Elizabeth".into())]
        );
    }
}
