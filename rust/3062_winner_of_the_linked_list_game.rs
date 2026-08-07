/// LeetCode #3062 - Winner of the Linked List Game

fn game_winner(values: Vec<i32>) -> String {
    let mut even = 0;
    let mut odd = 0;

    for i in (0..values.len()).step_by(2) {
        if values[i] > values[i + 1] {
            even += 1;
        } else {
            odd += 1;
        }
    }

    if even > odd {
        "Even".into()
    } else if odd > even {
        "Odd".into()
    } else {
        "Tie".into()
    }
}

fn main() {
    println!("{}", game_winner(vec![2, 1]));
    println!("{}", game_winner(vec![2, 5, 4, 7, 20, 5]));
    println!("{}", game_winner(vec![4, 5, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::game_winner;

    #[test]
    fn example1() {
        assert_eq!(game_winner(vec![2, 1]), "Even");
    }

    #[test]
    fn example2() {
        assert_eq!(game_winner(vec![2, 5, 4, 7, 20, 5]), "Odd");
    }

    #[test]
    fn example3() {
        assert_eq!(game_winner(vec![4, 5, 2, 1]), "Tie");
    }
}
