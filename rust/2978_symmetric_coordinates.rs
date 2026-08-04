/// LeetCode #2978 - Symmetric Coordinates (SQL; Rust analogue)
fn symmetric_coordinates(coords: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let n = coords.len();
    let mut ans = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            if x1 == y2 && y1 == x2 && x1 <= y1 {
                if seen.insert((x1, y1)) {
                    ans.push((x1, y1));
                }
            }
        }
    }
    ans.sort_unstable();
    ans
}

fn main() {
    println!(
        "{:?}",
        symmetric_coordinates(vec![
            (20, 20),
            (20, 20),
            (20, 21),
            (23, 22),
            (22, 23),
            (21, 20)
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::symmetric_coordinates;

    #[test]
    fn example() {
        assert_eq!(
            symmetric_coordinates(vec![
                (20, 20),
                (20, 20),
                (20, 21),
                (23, 22),
                (22, 23),
                (21, 20)
            ]),
            vec![(20, 20), (20, 21), (22, 23)]
        );
    }
}
