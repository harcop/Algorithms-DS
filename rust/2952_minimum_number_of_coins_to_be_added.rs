/// LeetCode #2952 - Minimum Number of Coins to be Added
fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
    coins.sort_unstable();
    let mut s = 1i64;
    let mut ans = 0;
    let mut i = 0usize;
    let target = target as i64;
    while s <= target {
        if i < coins.len() && coins[i] as i64 <= s {
            s += coins[i] as i64;
            i += 1;
        } else {
            s <<= 1;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_added_coins(vec![1, 4, 10], 19));
}

#[cfg(test)]
mod tests {
    use super::minimum_added_coins;

    #[test]
    fn example_one() {
        assert_eq!(minimum_added_coins(vec![1, 4, 10], 19), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_added_coins(vec![1, 1, 1], 20), 3);
    }
}
