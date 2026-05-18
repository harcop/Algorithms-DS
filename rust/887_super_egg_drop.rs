/// LeetCode #887 - Super Egg Drop
fn super_egg_drop(k: i32, n: i32) -> i32 {
    let k = k as usize;
    let n = n as usize;
    let mut dp = vec![0i32; k + 1];
    let mut moves = 0;
    while dp[k] < n as i32 {
        moves += 1;
        for i in (1..=k).rev() {
            dp[i] += dp[i - 1] + 1;
        }
    }
    moves
}

fn main() {
    println!("{}", super_egg_drop(1, 2));
}

#[cfg(test)]
mod tests {
    use super::super_egg_drop;

    #[test]
    fn example_one() {
        assert_eq!(super_egg_drop(1, 2), 2);
    }
}
