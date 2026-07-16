/// LeetCode #2410 - Maximum Matching of Players With Trainers
fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    players.sort_unstable();
    trainers.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    let mut ans = 0;

    while i < players.len() && j < trainers.len() {
        if players[i] <= trainers[j] {
            ans += 1;
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    ans
}

fn main() {
    println!("{}", match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]));
}

#[cfg(test)]
mod tests {
    use super::match_players_and_trainers;

    #[test]
    fn example_one() {
        assert_eq!(match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(match_players_and_trainers(vec![1, 1, 1], vec![10]), 1);
    }
}
