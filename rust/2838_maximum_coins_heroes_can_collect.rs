/// LeetCode #2838 - Maximum Coins Heroes Can Collect
fn maximum_coins(heroes: Vec<i32>, monsters: Vec<i32>, coins: Vec<i32>) -> Vec<i64> {
    let mut monsters: Vec<_> = monsters.into_iter().zip(coins).collect();
    monsters.sort_unstable_by_key(|&(power, _)| power);

    let mut prefix = vec![0i64; monsters.len() + 1];
    for (i, &(_, coin)) in monsters.iter().enumerate() {
        prefix[i + 1] = prefix[i] + coin as i64;
    }

    heroes
        .into_iter()
        .map(|hero| {
            let mut left = 0;
            let mut right = monsters.len();
            while left < right {
                let middle = left + (right - left) / 2;
                if monsters[middle].0 <= hero {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }
            prefix[left]
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        maximum_coins(vec![1, 4, 2], vec![1, 1, 5, 2, 3], vec![2, 3, 4, 5, 6])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_coins;

    #[test]
    fn examples() {
        assert_eq!(
            maximum_coins(vec![1, 4, 2], vec![1, 1, 5, 2, 3], vec![2, 3, 4, 5, 6]),
            vec![5, 16, 10]
        );
        assert_eq!(
            maximum_coins(vec![5], vec![2, 3, 1, 2], vec![10, 6, 5, 2]),
            vec![23]
        );
        assert_eq!(
            maximum_coins(vec![4, 4], vec![5, 7, 8], vec![1, 1, 1]),
            vec![0, 0]
        );
    }
}
