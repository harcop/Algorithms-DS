/// LeetCode #3386 - Button with Longest Push Time
fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
    let mut ans = events[0][0];
    let mut t = events[0][1];
    for w in events.windows(2) {
        let d = w[1][1] - w[0][1];
        let i = w[1][0];
        if d > t || (d == t && i < ans) {
            ans = i;
            t = d;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        button_with_longest_time(vec![vec![1, 2], vec![2, 5], vec![3, 9], vec![1, 15]])
    );
}

#[cfg(test)]
mod tests {
    use super::button_with_longest_time;

    #[test]
    fn example1() {
        assert_eq!(
            button_with_longest_time(vec![vec![1, 2], vec![2, 5], vec![3, 9], vec![1, 15]]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            button_with_longest_time(vec![vec![10, 5], vec![1, 7]]),
            10
        );
    }
}
