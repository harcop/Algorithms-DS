/// LeetCode #1045 - Customers Who Bought All Products (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn customers_who_bought_all(
    customer: Vec<(i32, i32)>,
    product: Vec<i32>,
) -> Vec<i32> {
    let all: HashSet<i32> = product.into_iter().collect();
    let mut by_c: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (cid, pk) in customer {
        by_c.entry(cid).or_default().insert(pk);
    }
    let mut ans: Vec<i32> = by_c
        .into_iter()
        .filter(|(_, ps)| ps.is_superset(&all))
        .map(|(c, _)| c)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::customers_who_bought_all;

    #[test]
    fn example() {
        let customer = vec![(1, 5), (2, 6), (3, 5), (3, 6), (1, 6)];
        let product = vec![5, 6];
        assert_eq!(customers_who_bought_all(customer, product), vec![1, 3]);
    }
}
