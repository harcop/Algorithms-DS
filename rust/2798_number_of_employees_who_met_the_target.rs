/// LeetCode #2798 - Number of Employees Who Met the Target
fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.iter().filter(|&&x| x >= target).count() as i32
}

fn main() {
    println!("{}", number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::number_of_employees_who_met_target;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6),
            0
        );
    }
}
