/// LeetCode #986 - Interval List Intersections
fn interval_intersection(first: Vec<Vec<i32>>, second: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut out = Vec::new();
    while i < first.len() && j < second.len() {
        let lo = first[i][0].max(second[j][0]);
        let hi = first[i][1].min(second[j][1]);
        if lo <= hi {
            out.push(vec![lo, hi]);
        }
        if first[i][1] < second[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::interval_intersection;

    #[test]
    fn example_one() {
        assert_eq!(
            interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
            ),
            vec![vec![1, 2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]]
        );
    }
}
