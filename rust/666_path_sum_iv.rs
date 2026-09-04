/// LeetCode #666 - Path Sum IV
use std::collections::HashMap;

fn path_sum(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for x in nums {
        let d = x / 100;
        let p = (x / 10) % 10;
        let v = x % 10;
        map.insert((d, p), v);
    }
    let mut ans = 0;
    fn dfs(map: &HashMap<(i32, i32), i32>, d: i32, p: i32, acc: i32, ans: &mut i32) {
        let Some(&v) = map.get(&(d, p)) else {
            return;
        };
        let acc = acc + v;
        let left = (d + 1, p * 2 - 1);
        let right = (d + 1, p * 2);
        if !map.contains_key(&left) && !map.contains_key(&right) {
            *ans += acc;
            return;
        }
        dfs(map, left.0, left.1, acc, ans);
        dfs(map, right.0, right.1, acc, ans);
    }
    dfs(&map, 1, 1, 0, &mut ans);
    ans
}

fn main() {
    println!("{}", path_sum(vec![113, 215, 221]));
}

#[cfg(test)]
mod tests {
    use super::path_sum;

    #[test]
    fn example_one() {
        assert_eq!(path_sum(vec![113, 215, 221]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(path_sum(vec![113, 221]), 4);
    }
}
