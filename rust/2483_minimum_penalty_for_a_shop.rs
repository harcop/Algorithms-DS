/// LeetCode #2483 - Minimum Penalty for a Shop
fn best_closing_time(customers: String) -> i32 {
    let mut answer = 0;
    let mut profit = 0;
    let mut max_profit = 0;

    for (index, customer) in customers.chars().enumerate() {
        profit += if customer == 'Y' { 1 } else { -1 };
        if profit > max_profit {
            max_profit = profit;
            answer = (index + 1) as i32;
        }
    }

    answer
}

fn main() {
    println!("{}", best_closing_time("YYNY".to_string()));
}

#[cfg(test)]
mod tests {
    use super::best_closing_time;

    #[test]
    fn example_one() {
        assert_eq!(best_closing_time("YYNY".to_string()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(best_closing_time("NNNNN".to_string()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(best_closing_time("YYYY".to_string()), 4);
    }
}
