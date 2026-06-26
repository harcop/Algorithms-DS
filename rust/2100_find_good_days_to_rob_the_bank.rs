/// LeetCode #2100 - Find Good Days to Rob the Bank
fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let n = security.len();
    let time = time as usize;
    let mut non_inc = vec![0usize; n];
    let mut non_dec = vec![0usize; n];

    for i in 1..n {
        if security[i - 1] >= security[i] {
            non_inc[i] = non_inc[i - 1] + 1;
        }
    }
    for i in (0..n.saturating_sub(1)).rev() {
        if security[i] <= security[i + 1] {
            non_dec[i] = non_dec[i + 1] + 1;
        }
    }

    (0..n)
        .filter(|&i| non_inc[i] >= time && non_dec[i] >= time)
        .map(|i| i as i32)
        .collect()
}

fn main() {
    println!("{:?}", good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::good_days_to_rob_bank;

    #[test]
    fn example_one() {
        assert_eq!(
            good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2),
            vec![2, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn example_three() {
        assert_eq!(good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2), Vec::<i32>::new());
    }
}
