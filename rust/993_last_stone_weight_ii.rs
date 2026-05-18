/// LeetCode #993 - Last Stone Weight II
fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();
    let mut dp = vec![false; (sum as usize / 2) + 1];
    dp[0] = true;
    for s in stones {
        for j in (s as usize..=sum as usize / 2).rev() {
            dp[j] = dp[j] || dp[j - s as usize];
        }
    }
    for half in (0..=sum as usize / 2).rev() {
        if dp[half] {
            return sum - 2 * half as i32;
        }
    }
    0
}

fn main() {
    println!("{}", last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
}

#[cfg(test)]
mod tests {
    use super::last_stone_weight_ii;

    #[test]
    fn example_one() {
        assert_eq!(last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
