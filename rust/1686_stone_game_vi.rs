/// LeetCode #1686 - Stone Game Vi
use std::cmp::Reverse;

fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut idx: Vec<usize> = (0..alice_values.len()).collect();
    idx.sort_unstable_by_key(|&i| Reverse(alice_values[i] + bob_values[i]));
    let (mut a, mut b) = (0i64, 0i64);
    for (turn, &i) in idx.iter().enumerate() {
        if turn % 2 == 0 { a += alice_values[i] as i64; } else { b += bob_values[i] as i64; }
    }
    if a > b { 1 } else if a < b { -1 } else { 0 }
}
fn main() { println!("{}", stone_game_vi(vec![1,3], vec![2,4])); }
#[cfg(test)]
mod tests {
    use super::stone_game_vi;
    #[test]
    fn example_one() { assert_eq!(stone_game_vi(vec![1,3], vec![2,4]), 1); }
}