/// LeetCode #1518 - Water Bottles
fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut full = num_bottles;
    let mut empty = 0;
    let mut drunk = 0;
    while full > 0 {
        drunk += full;
        empty += full;
        full = empty / num_exchange;
        empty %= num_exchange;
    }
    drunk
}

fn main() {
    println!("{}", num_water_bottles(9, 3));
}

#[cfg(test)]
mod tests {
    use super::num_water_bottles;

    #[test]
    fn example_one() {
        assert_eq!(num_water_bottles(9, 3), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_water_bottles(15, 4), 19);
    }
}
