/// LeetCode #2739 - Total Distance Traveled
fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
    let mut cur = 0;
    let mut ans = 0;
    while main_tank > 0 {
        cur += 1;
        main_tank -= 1;
        ans += 10;
        if cur % 5 == 0 && additional_tank > 0 {
            additional_tank -= 1;
            main_tank += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", distance_traveled(5, 10));
}

#[cfg(test)]
mod tests {
    use super::distance_traveled;

    #[test]
    fn example_one() {
        assert_eq!(distance_traveled(5, 10), 60);
    }

    #[test]
    fn example_two() {
        assert_eq!(distance_traveled(1, 2), 10);
    }
}
