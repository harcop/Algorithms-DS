/// LeetCode #1431 - Kids With The Greatest Number Of Candies
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_c = *candies.iter().max().unwrap();
    candies.into_iter().map(|c| c + extra_candies >= max_c).collect()
}

fn main() {
    println!("{:?}", kids_with_candies(vec![2, 3, 5, 1, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::kids_with_candies;

    #[test]
    fn example_one() {
        assert_eq!(kids_with_candies(vec![2, 3, 5, 1, 3], 3), vec![true, true, true, false, true]);
    }

    #[test]
    fn example_two() {
        assert_eq!(kids_with_candies(vec![4, 2, 1, 1, 2], 1), vec![true, false, false, false, false]);
    }
}

