/// LeetCode #2367 - Number of Arithmetic Triplets
fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut seen = [false; 201];
    let mut ans = 0;

    for &num in &nums {
        if num >= 2 * diff && seen[(num - diff) as usize] && seen[(num - 2 * diff) as usize] {
            ans += 1;
        }
        seen[num as usize] = true;
    }

    ans
}

fn main() {
    println!("{}", arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3));
}

#[cfg(test)]
mod tests {
    use super::arithmetic_triplets;

    #[test]
    fn example_one() {
        assert_eq!(arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
    }
}
