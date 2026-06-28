/// LeetCode #2139 - Minimum Moves to Reach Target Score
fn min_moves(mut target: i32, mut max_doubles: i32) -> i32 {
    let mut moves = 0;

    while target > 1 && max_doubles > 0 {
        if target % 2 == 0 {
            target /= 2;
            max_doubles -= 1;
        } else {
            target -= 1;
        }
        moves += 1;
    }

    moves + target - 1
}

fn main() {
    println!("{}", min_moves(5, 0));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example_one() {
        assert_eq!(min_moves(5, 0), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_moves(19, 2), 7);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_moves(10, 4), 4);
    }
}
