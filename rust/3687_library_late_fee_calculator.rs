/// LeetCode #3687 - Library Late Fee Calculator (premium)
fn late_fee(days_late: Vec<i32>) -> i32 {
    days_late
        .into_iter()
        .map(|x| {
            if x == 1 {
                1
            } else if x > 5 {
                3 * x
            } else {
                2 * x
            }
        })
        .sum()
}

fn main() {
    println!("{}", late_fee(vec![5, 1, 7]));
}

#[cfg(test)]
mod tests {
    use super::late_fee;

    #[test]
    fn example1() {
        assert_eq!(late_fee(vec![5, 1, 7]), 32);
    }

    #[test]
    fn example2() {
        assert_eq!(late_fee(vec![1, 1]), 2);
    }
}
