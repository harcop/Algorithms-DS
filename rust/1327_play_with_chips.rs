/// LeetCode #1327 - Play with Chips
fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
    let mut even = 0;
    let mut odd = 0;
    for c in chips {
        if c % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    even.min(odd)
}

fn main() {
    println!("{}", min_cost_to_move_chips(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_cost_to_move_chips;

    #[test]
    fn example_one() {
        assert_eq!(min_cost_to_move_chips(vec![1, 2, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    }
}
