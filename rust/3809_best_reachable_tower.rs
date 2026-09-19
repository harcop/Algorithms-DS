/// LeetCode #3809 - Best Reachable Tower
fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
    let cx = center[0];
    let cy = center[1];
    let mut idx: Option<usize> = None;
    for (i, t) in towers.iter().enumerate() {
        let x = t[0];
        let y = t[1];
        let q = t[2];
        let dist = (x - cx).abs() + (y - cy).abs();
        if dist > radius {
            continue;
        }
        let better = match idx {
            None => true,
            Some(j) => {
                let bq = towers[j][2];
                q > bq || (q == bq && (x < towers[j][0] || (x == towers[j][0] && y < towers[j][1])))
            }
        };
        if better {
            idx = Some(i);
        }
    }
    match idx {
        None => vec![-1, -1],
        Some(i) => vec![towers[i][0], towers[i][1]],
    }
}

fn main() {
    println!(
        "{:?}",
        best_tower(
            vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]],
            vec![1, 1],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::best_tower;

    #[test]
    fn example1() {
        assert_eq!(
            best_tower(
                vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]],
                vec![1, 1],
                2
            ),
            vec![3, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            best_tower(
                vec![vec![1, 3, 4], vec![2, 2, 4], vec![4, 4, 7]],
                vec![0, 0],
                5
            ),
            vec![1, 3]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            best_tower(vec![vec![5, 6, 8], vec![0, 3, 5]], vec![1, 2], 1),
            vec![-1, -1]
        );
    }
}
