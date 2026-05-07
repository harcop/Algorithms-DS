/// LeetCode #464 - Can I Win
use std::collections::HashMap;

fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if desired_total <= 0 {
        return false;
    }
    let max = max_choosable_integer as usize;
    let sum_all = max * (max + 1) / 2;
    if desired_total <= max as i32 {
        return true;
    }
    if sum_all < desired_total as usize {
        return false;
    }
    let mut memo: HashMap<u32, bool> = HashMap::new();

    fn dfs(mask: u32, total: i32, desired: i32, max: usize, memo: &mut HashMap<u32, bool>) -> bool {
        if let Some(&v) = memo.get(&mask) {
            return v;
        }
        for i in 1..=max {
            let bit = 1u32 << (i - 1);
            if mask & bit != 0 {
                continue;
            }
            if total + i as i32 >= desired || !dfs(mask | bit, total + i as i32, desired, max, memo) {
                memo.insert(mask, true);
                return true;
            }
        }
        memo.insert(mask, false);
        false
    }

    dfs(0, 0, desired_total, max, &mut memo)
}

fn main() {
    println!("{}", can_i_win(10, 1));
}

#[cfg(test)]
mod tests {
    use super::can_i_win;

    #[test]
    fn example_one() {
        assert!(!can_i_win(10, 11));
    }

    #[test]
    fn example_two() {
        assert!(!can_i_win(10, 0));
    }

    #[test]
    fn example_three() {
        assert!(can_i_win(10, 1));
    }
}
