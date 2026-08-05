/// LeetCode #2998 - Minimum Number of Operations to Make X and Y Equal
use std::collections::HashMap;

fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
    fn dfs(x: i32, y: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if y >= x {
            return y - x;
        }
        if let Some(&ans) = memo.get(&x) {
            return ans;
        }
        let mut ans = x - y;
        ans = ans.min(x % 5 + 1 + dfs(x / 5, y, memo));
        ans = ans.min(5 - x % 5 + 1 + dfs(x / 5 + 1, y, memo));
        ans = ans.min(x % 11 + 1 + dfs(x / 11, y, memo));
        ans = ans.min(11 - x % 11 + 1 + dfs(x / 11 + 1, y, memo));
        memo.insert(x, ans);
        ans
    }
    let mut memo = HashMap::new();
    dfs(x, y, &mut memo)
}

fn main() {
    println!("{}", minimum_operations_to_make_equal(26, 1));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations_to_make_equal;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations_to_make_equal(26, 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations_to_make_equal(54, 2), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_operations_to_make_equal(25, 30), 5);
    }
}
