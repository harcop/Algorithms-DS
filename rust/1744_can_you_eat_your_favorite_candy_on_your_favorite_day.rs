/// LeetCode #1744 - Can You Eat Your Favorite Candy on Your Favorite Day?
fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut prefix = vec![0i64];
    for c in candies_count {
        prefix.push(prefix.last().unwrap() + c as i64);
    }
    queries
        .into_iter()
        .map(|q| {
            let t = q[0] as usize;
            let day = q[1] as i64;
            let cap = q[2] as i64;
            let least = day;
            let most = (day + 1) * cap;
            least < prefix[t + 1] && most > prefix[t]
        })
        .collect()
}
fn main() {
    println!(
        "{:?}",
        can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1_000_000_000]],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::can_eat;
    #[test]
    fn example_one() {
        assert_eq!(
            can_eat(
                vec![7, 4, 5, 3, 8],
                vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1_000_000_000]],
            ),
            vec![true, false, true]
        );
    }
}
