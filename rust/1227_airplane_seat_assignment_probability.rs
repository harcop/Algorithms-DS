/// LeetCode #1227 - Airplane Seat Assignment Probability
fn nth_person_gets_nth_seat(n: i32) -> f64 {
    if n == 1 {
        1.0
    } else {
        0.5
    }
}

fn main() {
    println!("{}", nth_person_gets_nth_seat(2));
}

#[cfg(test)]
mod tests {
    use super::nth_person_gets_nth_seat;

    #[test]
    fn example_one() {
        assert!((nth_person_gets_nth_seat(1) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        assert!((nth_person_gets_nth_seat(2) - 0.5).abs() < 1e-5);
    }

    #[test]
    fn example_three() {
        assert!((nth_person_gets_nth_seat(5) - 0.5).abs() < 1e-5);
    }
}
