/// LeetCode #1402 - Reducing Dishes
fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut s = satisfaction;
    s.sort_unstable();
    let mut suffix = 0i32;
    let mut total = 0i32;
    let mut ans = 0i32;
    for i in (0..s.len()).rev() {
        suffix += s[i];
        total += suffix;
        ans = ans.max(total);
    }
    ans
}

fn main() {
    println!("{}", max_satisfaction(vec![-1, -8, 0, 5, -9]));
}

#[cfg(test)]
mod tests {
    use super::max_satisfaction;

    #[test]
    fn example_one() {
        assert_eq!(max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_satisfaction(vec![4, 3, 2]), 20);
    }
}

