/// LeetCode #1560 - Most Visited Sector In A Circular Track
fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    let start = rounds[0];
    let end = *rounds.last().unwrap();
    if start <= end {
        (start..=end).collect()
    } else {
        let mut ans: Vec<i32> = (start..=n).collect();
        ans.extend(1..=end);
        ans
    }
}

fn main() {
    println!("{:?}", most_visited(4, vec![1, 3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::most_visited;

    #[test]
    fn example_one() {
        assert_eq!(most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]), vec![2]);
    }
}
