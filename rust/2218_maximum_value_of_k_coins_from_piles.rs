/// LeetCode #2218 - Maximum Value of K Coins From Piles
fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let n = piles.len();
    let mut mem = vec![vec![-1i32; k + 1]; n];
    dp(&piles, 0, k, &mut mem)
}

fn dp(piles: &[Vec<i32>], i: usize, k: usize, mem: &mut [Vec<i32>]) -> i32 {
    if i == piles.len() || k == 0 {
        return 0;
    }
    if mem[i][k] != -1 {
        return mem[i][k];
    }

    let mut res = dp(piles, i + 1, k, mem);
    let mut val = 0i32;
    let take = piles[i].len().min(k);
    for j in 0..take {
        val += piles[i][j];
        res = res.max(val + dp(piles, i + 1, k - j - 1, mem));
    }

    mem[i][k] = res;
    res
}

fn main() {
    println!(
        "{}",
        max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::max_value_of_coins;

    #[test]
    fn example_one() {
        assert_eq!(
            max_value_of_coins(vec![vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_value_of_coins(
                vec![
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![1, 1, 1, 1, 1, 1, 700],
                ],
                7
            ),
            706
        );
    }
}
