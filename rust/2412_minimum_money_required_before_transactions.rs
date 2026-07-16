/// LeetCode #2412 - Minimum Money Required Before Transactions
fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
    let mut loss = 0i64;
    let mut need = 0i64;

    for transaction in transactions {
        let cost = transaction[0] as i64;
        let cashback = transaction[1] as i64;
        loss += (cost - cashback).max(0);
        need = need.max(cost.min(cashback));
    }

    loss + need
}

fn main() {
    println!("{}", minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]));
}

#[cfg(test)]
mod tests {
    use super::minimum_money;

    #[test]
    fn example_one() {
        assert_eq!(minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_money(vec![vec![3, 0], vec![0, 3]]), 3);
    }
}
