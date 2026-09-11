/// LeetCode #3701 - Compute Alternating Sum
fn alternating_sum(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .map(|(i, &x)| if i % 2 == 0 { x } else { -x })
        .sum()
}

fn main() {
    println!("{}", alternating_sum(vec![1, 3, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::alternating_sum;

    #[test]
    fn example1() {
        assert_eq!(alternating_sum(vec![1, 3, 5, 7]), -4);
    }

    #[test]
    fn example2() {
        assert_eq!(alternating_sum(vec![100]), 100);
    }
}
