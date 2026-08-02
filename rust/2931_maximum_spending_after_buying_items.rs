/// LeetCode #2931 - Maximum Spending After Buying Items
fn max_spending(values: Vec<Vec<i32>>) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let m = values[0].len();
    let mut pq = BinaryHeap::new();
    for (i, row) in values.iter().enumerate() {
        pq.push(Reverse((row[m - 1], i, m - 1)));
    }

    let mut ans = 0i64;
    let mut day = 0i64;
    while let Some(Reverse((v, i, j))) = pq.pop() {
        day += 1;
        ans += v as i64 * day;
        if j > 0 {
            pq.push(Reverse((values[i][j - 1], i, j - 1)));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_spending(vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_spending;

    #[test]
    fn example_one() {
        assert_eq!(
            max_spending(vec![vec![8, 5, 2], vec![6, 4, 1], vec![9, 7, 3]]),
            285
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_spending(vec![vec![10, 8, 6, 4, 2], vec![9, 7, 5, 3, 2]]),
            386
        );
    }
}
