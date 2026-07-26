/// LeetCode #2682 - Find the Losers of the Circular Game
fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut vis = vec![false; n];
    let mut i = 0;
    let mut p = 1;
    while !vis[i] {
        vis[i] = true;
        i = (i + p * k) % n;
        p += 1;
    }
    (0..n)
        .filter(|&i| !vis[i])
        .map(|i| (i + 1) as i32)
        .collect()
}

fn main() {
    println!("{:?}", circular_game_losers(5, 2));
}

#[cfg(test)]
mod tests {
    use super::circular_game_losers;

    #[test]
    fn example_one() {
        assert_eq!(circular_game_losers(5, 2), vec![4, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(circular_game_losers(4, 4), vec![2, 3, 4]);
    }
}
