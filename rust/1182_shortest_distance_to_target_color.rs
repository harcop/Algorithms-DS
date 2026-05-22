/// LeetCode #1182 - Shortest Distance to Target Color
fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = colors.len();
    let mut dist = vec![vec![n; 4]; n];
    for c in 1..4usize {
        let mut last = n;
        for i in 0..n {
            if colors[i] as usize == c {
                last = i;
            }
            if last < n {
                dist[i][c] = dist[i][c].min(i - last);
            }
        }
        last = n;
        for i in (0..n).rev() {
            if colors[i] as usize == c {
                last = i;
            }
            if last < n {
                dist[i][c] = dist[i][c].min(last - i);
            }
        }
    }
    queries
        .into_iter()
        .map(|q| dist[q[0] as usize][q[1] as usize] as i32)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        shortest_distance_color(
            vec![1, 1, 2, 1, 3, 2, 2, 3, 3],
            vec![vec![1, 3], vec![2, 2]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_distance_color;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_distance_color(
                vec![1, 1, 2, 1, 3, 2, 2, 3, 3],
                vec![vec![1, 3], vec![2, 2]],
            ),
            vec![3, 0]
        );
    }
}
