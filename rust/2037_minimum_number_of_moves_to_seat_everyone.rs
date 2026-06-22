/// LeetCode #2037 - Minimum Number of Moves to Seat Everyone
fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;
    seats.sort_unstable();
    students.sort_unstable();
    seats
        .iter()
        .zip(students)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn main() {
    println!("{}", min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_moves_to_seat;

    #[test]
    fn example_one() {
        assert_eq!(min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]), 7);
    }
}
