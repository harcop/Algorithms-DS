/// LeetCode #1655 - Distribute Repeating Integers
fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
    let mut cnt = [0i32; 101];
    for x in nums {
        cnt[x as usize] += 1;
    }
    let mut qty = quantity;
    qty.sort_unstable_by(|a, b| b.cmp(a));

    fn dfs(cnt: &mut [i32; 101], qty: &[i32], i: usize) -> bool {
        if i == qty.len() {
            return true;
        }
        let need = qty[i];
        let mut seen = std::collections::HashSet::new();
        for v in 1..=100 {
            if cnt[v] >= need && seen.insert(cnt[v]) {
                cnt[v] -= need;
                if dfs(cnt, qty, i + 1) {
                    return true;
                }
                cnt[v] += need;
            }
        }
        false
    }

    dfs(&mut cnt, &qty, 0)
}

fn main() {
    println!("{}", can_distribute(vec![1, 1, 2, 2], vec![2, 2]));
}

#[cfg(test)]
mod tests {
    use super::can_distribute;

    #[test]
    fn example_one() {
        assert!(!can_distribute(vec![1, 2, 3, 4], vec![2]));
    }

    #[test]
    fn example_two() {
        assert!(can_distribute(vec![1, 1, 2, 2], vec![2, 2]));
    }
}
