/// LeetCode #3155 - Maximum Number of Upgradable Servers
fn max_upgrades(
    count: Vec<i32>,
    upgrade: Vec<i32>,
    sell: Vec<i32>,
    money: Vec<i32>,
) -> Vec<i32> {
    count
        .into_iter()
        .zip(upgrade)
        .zip(sell)
        .zip(money)
        .map(|(((cnt, cost), income), cash)| {
            let cnt = cnt as i64;
            let cost = cost as i64;
            let income = income as i64;
            let cash = cash as i64;
            cnt.min((cnt * income + cash) / (cost + income)) as i32
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        max_upgrades(vec![4, 3], vec![3, 5], vec![4, 2], vec![8, 9])
    );
}

#[cfg(test)]
mod tests {
    use super::max_upgrades;

    #[test]
    fn example1() {
        assert_eq!(
            max_upgrades(vec![4, 3], vec![3, 5], vec![4, 2], vec![8, 9]),
            vec![3, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_upgrades(vec![1], vec![2], vec![1], vec![1]),
            vec![0]
        );
    }
}
