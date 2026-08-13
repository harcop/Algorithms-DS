/// LeetCode #3189 - Minimum Moves to Get a Peaceful Board
fn min_moves(mut rooks: Vec<Vec<i32>>) -> i32 {
    rooks.sort_by_key(|r| r[0]);
    let mut ans: i32 = rooks
        .iter()
        .enumerate()
        .map(|(i, r)| (r[0] - i as i32).abs())
        .sum();
    rooks.sort_by_key(|r| r[1]);
    ans += rooks
        .iter()
        .enumerate()
        .map(|(j, r)| (r[1] - j as i32).abs())
        .sum::<i32>();
    ans
}

fn main() {
    println!("{}", min_moves(vec![vec![0, 0], vec![1, 0], vec![1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(vec![vec![0, 0], vec![1, 0], vec![1, 1]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_moves(vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]),
            6
        );
    }
}
