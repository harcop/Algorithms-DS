/// LeetCode #638 - Shopping Offers
use std::collections::HashMap;

fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    fn dfs(
        price: &[i32],
        special: &[Vec<i32>],
        needs: Vec<i32>,
        memo: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if let Some(&v) = memo.get(&needs) {
            return v;
        }
        let mut cost: i32 = needs.iter().zip(price.iter()).map(|(n, p)| n * p).sum();
        for offer in special {
            let mut next = needs.clone();
            let mut ok = true;
            for i in 0..needs.len() {
                if offer[i] > next[i] {
                    ok = false;
                    break;
                }
                next[i] -= offer[i];
            }
            if ok {
                cost = cost.min(offer[needs.len()] + dfs(price, special, next, memo));
            }
        }
        memo.insert(needs, cost);
        cost
    }
    dfs(&price, &special, needs, &mut HashMap::new())
}

fn main() {
    let price = vec![2, 5];
    let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
    let needs = vec![3, 2];
    println!("{}", shopping_offers(price, special, needs));
}

#[cfg(test)]
mod tests {
    use super::shopping_offers;

    #[test]
    fn example_one() {
        let price = vec![2, 5];
        let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
        let needs = vec![3, 2];
        assert_eq!(shopping_offers(price, special, needs), 14);
    }

    #[test]
    fn example_two() {
        let price = vec![2, 3, 4];
        let special = vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]];
        let needs = vec![1, 2, 1];
        assert_eq!(shopping_offers(price, special, needs), 11);
    }
}
