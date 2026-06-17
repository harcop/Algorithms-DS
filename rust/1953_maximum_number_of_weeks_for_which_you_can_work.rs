/// LeetCode #1953 - Maximum Number of Weeks for Which You Can Work
fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mx = *milestones.iter().max().unwrap() as i64;
    let s: i64 = milestones.iter().map(|&x| x as i64).sum();
    let rest = s - mx;
    if mx > rest + 1 {
        rest * 2 + 1
    } else {
        s
    }
}

fn main() {
    println!("{}", number_of_weeks(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::number_of_weeks;

    #[test]
    fn example_one() {
        assert_eq!(number_of_weeks(vec![1, 2, 3]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_weeks(vec![5, 2, 1]), 7);
    }
}
