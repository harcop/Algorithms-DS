/// LeetCode #2374 - Node With Highest Edge Score
fn edge_score(edges: Vec<i32>) -> i32 {
    let n = edges.len();
    let mut cnt = vec![0i64; n];
    let mut ans = 0usize;

    for (i, &j) in edges.iter().enumerate() {
        let j = j as usize;
        cnt[j] += i as i64;
        if cnt[ans] < cnt[j] || (cnt[ans] == cnt[j] && j < ans) {
            ans = j;
        }
    }

    ans as i32
}

fn main() {
    println!("{}", edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]));
}

#[cfg(test)]
mod tests {
    use super::edge_score;

    #[test]
    fn example_one() {
        assert_eq!(edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(edge_score(vec![2, 0, 0, 2]), 0);
    }
}
