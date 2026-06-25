/// LeetCode #2073 - Time Needed to Buy Tickets
fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    tickets
        .iter()
        .enumerate()
        .map(|(i, &ticket)| {
            if i <= k {
                ticket.min(tickets[k])
            } else {
                ticket.min(tickets[k] - 1)
            }
        })
        .sum()
}

fn main() {
    println!("{}", time_required_to_buy(vec![2, 3, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::time_required_to_buy;

    #[test]
    fn example_one() {
        assert_eq!(time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
