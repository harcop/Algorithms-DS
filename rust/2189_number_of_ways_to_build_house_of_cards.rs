/// LeetCode #2189 - Number of Ways to Build House of Cards
use std::collections::HashMap;

fn house_of_cards(n: i32) -> i32 {
    let n = n as i32;
    let mut memo = HashMap::new();
    fn dfs(n: i32, k: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if let Some(&ans) = memo.get(&(n, k)) {
            return ans;
        }
        let need = 3 * k + 2;
        let ans = if need > n {
            0
        } else if need == n {
            1
        } else {
            dfs(n - need, k + 1, memo) + dfs(n, k + 1, memo)
        };
        memo.insert((n, k), ans);
        ans
    }
    dfs(n, 0, &mut memo)
}

fn main() {
    println!("{}", house_of_cards(16));
}

#[cfg(test)]
mod tests {
    use super::house_of_cards;

    #[test]
    fn example_one() {
        assert_eq!(house_of_cards(16), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(house_of_cards(4), 0);
    }

    #[test]
    fn single_row() {
        assert_eq!(house_of_cards(2), 1);
    }
}
