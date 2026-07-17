/// LeetCode #2438 - Range Product Queries of Powers
fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;

    let mut powers = Vec::new();
    for bit in 0..31 {
        if n & (1 << bit) != 0 {
            powers.push(1i64 << bit);
        }
    }

    queries
        .into_iter()
        .map(|query| {
            powers[query[0] as usize..=query[1] as usize]
                .iter()
                .fold(1i64, |product, &power| product * power % MOD) as i32
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::product_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]),
            vec![2, 4, 64]
        );
    }

    #[test]
    fn single_power() {
        assert_eq!(product_queries(2, vec![vec![0, 0]]), vec![2]);
    }
}
