/// LeetCode #1140 - Stone Game II
fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let mut suffix = vec![0i32; n + 1];
    for i in (0..n).rev() {
        suffix[i] = suffix[i + 1] + piles[i];
    }
    const UNSET: i32 = -1;
    let mut alice_memo = vec![vec![UNSET; n + 1]; n];
    let mut bob_memo = vec![vec![UNSET; n + 1]; n];

    fn alice(
        i: usize,
        m: usize,
        suffix: &[i32],
        n: usize,
        alice_memo: &mut [Vec<i32>],
        bob_memo: &mut [Vec<i32>],
    ) -> i32 {
        if i >= n {
            return 0;
        }
        if alice_memo[i][m] != UNSET {
            return alice_memo[i][m];
        }
        let mut best = 0i32;
        let take_max = (2 * m).min(n - i);
        for x in 1..=take_max {
            let gain = suffix[i] - suffix[i + x];
            let bob_score = bob(i + x, x.max(m), suffix, n, alice_memo, bob_memo);
            best = best.max(gain + suffix[i + x] - bob_score);
        }
        alice_memo[i][m] = best;
        best
    }

    fn bob(
        i: usize,
        m: usize,
        suffix: &[i32],
        n: usize,
        alice_memo: &mut [Vec<i32>],
        bob_memo: &mut [Vec<i32>],
    ) -> i32 {
        if i >= n {
            return 0;
        }
        if bob_memo[i][m] != UNSET {
            return bob_memo[i][m];
        }
        let mut best = 0i32;
        let take_max = (2 * m).min(n - i);
        for x in 1..=take_max {
            let gain = suffix[i] - suffix[i + x];
            let alice_score = alice(i + x, x.max(m), suffix, n, alice_memo, bob_memo);
            best = best.max(gain + suffix[i + x] - alice_score);
        }
        bob_memo[i][m] = best;
        best
    }

    alice(0, 1, &suffix, n, &mut alice_memo, &mut bob_memo)
}

fn main() {
    println!("{}", stone_game_ii(vec![2, 7, 9, 4, 4]));
}

#[cfg(test)]
mod tests {
    use super::stone_game_ii;

    #[test]
    fn example_one() {
        assert_eq!(stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
