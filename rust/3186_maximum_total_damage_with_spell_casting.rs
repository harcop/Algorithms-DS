/// LeetCode #3186 - Maximum Total Damage With Spell Casting
use std::collections::HashMap;

fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
    power.sort_unstable();
    let n = power.len();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &p in &power {
        *cnt.entry(p).or_insert(0) += 1;
    }
    let mut nxt = vec![0usize; n];
    for i in 0..n {
        let target = power[i] + 3;
        nxt[i] = i + 1 + power[i + 1..].partition_point(|&x| x < target);
    }
    let mut memo = vec![-1i64; n];
    fn dfs(
        i: usize,
        n: usize,
        power: &[i32],
        nxt: &[usize],
        memo: &mut [i64],
        cnt: &HashMap<i32, i32>,
    ) -> i64 {
        if i >= n {
            return 0;
        }
        if memo[i] != -1 {
            return memo[i];
        }
        let c = *cnt.get(&power[i]).unwrap();
        let skip = dfs(i + c as usize, n, power, nxt, memo, cnt);
        let take = power[i] as i64 * c as i64 + dfs(nxt[i], n, power, nxt, memo, cnt);
        memo[i] = skip.max(take);
        memo[i]
    }
    dfs(0, n, &power, &nxt, &mut memo, &cnt)
}

fn main() {
    println!("{}", maximum_total_damage(vec![1, 1, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_total_damage;

    #[test]
    fn example1() {
        assert_eq!(maximum_total_damage(vec![1, 1, 3, 4]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }
}
