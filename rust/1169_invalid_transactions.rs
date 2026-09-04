/// LeetCode #1169 - Invalid Transactions
fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
    struct Tx {
        raw: String,
        name: String,
        time: i32,
        amount: i32,
        city: String,
    }
    let txs: Vec<Tx> = transactions
        .into_iter()
        .map(|raw| {
            let parts: Vec<&str> = raw.split(',').collect();
            Tx {
                name: parts[0].to_string(),
                time: parts[1].parse().unwrap(),
                amount: parts[2].parse().unwrap(),
                city: parts[3].to_string(),
                raw,
            }
        })
        .collect();
    let n = txs.len();
    let mut invalid = vec![false; n];
    for i in 0..n {
        if txs[i].amount > 1000 {
            invalid[i] = true;
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            if txs[i].name == txs[j].name
                && txs[i].city != txs[j].city
                && (txs[i].time - txs[j].time).abs() <= 60
            {
                invalid[i] = true;
            }
        }
    }
    txs.into_iter()
        .enumerate()
        .filter(|(i, _)| invalid[*i])
        .map(|(_, t)| t.raw)
        .collect()
}

fn main() {
    let t = vec!["alice,20,800,mtv".into(), "alice,50,100,beijing".into()];
    println!("{:?}", invalid_transactions(t));
}

#[cfg(test)]
mod tests {
    use super::invalid_transactions;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        let t = vec!["alice,20,800,mtv".into(), "alice,50,100,beijing".into()];
        assert_eq!(
            sorted(invalid_transactions(t)),
            sorted(vec![
                "alice,20,800,mtv".into(),
                "alice,50,100,beijing".into()
            ])
        );
    }

    #[test]
    fn example_two() {
        let t = vec!["alice,20,800,mtv".into(), "alice,50,1200,mtv".into()];
        assert_eq!(
            invalid_transactions(t),
            vec!["alice,50,1200,mtv".to_string()]
        );
    }

    #[test]
    fn example_three() {
        let t = vec!["alice,20,800,mtv".into(), "bob,50,1200,mtv".into()];
        assert_eq!(
            invalid_transactions(t),
            vec!["bob,50,1200,mtv".to_string()]
        );
    }
}
