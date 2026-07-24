/// LeetCode #2660 - Determine the Winner of a Bowling Game
fn score(arr: &[i32]) -> i32 {
    let mut s = 0;
    for i in 0..arr.len() {
        let k = if (i > 0 && arr[i - 1] == 10) || (i > 1 && arr[i - 2] == 10) {
            2
        } else {
            1
        };
        s += k * arr[i];
    }
    s
}

fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    let a = score(&player1);
    let b = score(&player2);
    if a > b {
        1
    } else if b > a {
        2
    } else {
        0
    }
}

fn main() {
    println!("{}", is_winner(vec![5, 10, 3, 2], vec![6, 5, 7, 3]));
}

#[cfg(test)]
mod tests {
    use super::is_winner;

    #[test]
    fn example_one() {
        assert_eq!(is_winner(vec![5, 10, 3, 2], vec![6, 5, 7, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(is_winner(vec![2, 3], vec![4, 1]), 0);
    }

    #[test]
    fn example_four() {
        assert_eq!(
            is_winner(
                vec![1, 1, 1, 10, 10, 10, 10],
                vec![10, 10, 10, 10, 1, 1, 1]
            ),
            2
        );
    }
}
