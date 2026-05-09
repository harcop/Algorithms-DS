/// LeetCode #554 - Brick Wall
use std::collections::HashMap;

fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for row in &wall {
        let mut pos = 0;
        for &b in row.iter().take(row.len().saturating_sub(1)) {
            pos += b;
            *cnt.entry(pos).or_insert(0) += 1;
        }
    }
    let max_edge = cnt.values().copied().max().unwrap_or(0);
    wall.len() as i32 - max_edge
}

fn main() {
    println!(
        "{}",
        least_bricks(vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::least_bricks;

    #[test]
    fn example_one() {
        assert_eq!(
            least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1],
            ]),
            2
        );
    }
}
