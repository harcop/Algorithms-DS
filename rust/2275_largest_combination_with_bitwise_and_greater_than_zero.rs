/// LeetCode #2275 - Largest Combination With Bitwise AND Greater Than Zero
fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut ans = 0;
    for bit in 0..24 {
        let count = candidates.iter().filter(|&&c| (c >> bit) & 1 == 1).count() as i32;
        ans = ans.max(count);
    }
    ans
}

fn main() {
    println!("{}", largest_combination(vec![16, 17, 71, 62, 12, 24, 14]));
}

#[cfg(test)]
mod tests {
    use super::largest_combination;

    #[test]
    fn example_one() {
        assert_eq!(largest_combination(vec![16, 17, 71, 62, 12, 24, 14]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_combination(vec![8, 8]), 2);
    }
}
