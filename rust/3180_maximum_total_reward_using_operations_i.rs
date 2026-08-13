/// LeetCode #3180 - Maximum Total Reward Using Operations I
fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
    reward_values.sort_unstable();
    reward_values.dedup();
    let mx = *reward_values.last().unwrap() as usize;
    let mut f = vec![false; mx * 2];
    f[0] = true;
    let mut ans = 0usize;
    for &v in &reward_values {
        let v = v as usize;
        for x in 0..v {
            if f[x] {
                f[x + v] = true;
                ans = ans.max(x + v);
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", max_total_reward(vec![1, 1, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_total_reward;

    #[test]
    fn example1() {
        assert_eq!(max_total_reward(vec![1, 1, 3, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }
}
