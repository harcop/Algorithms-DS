/// LeetCode #3394 - Check if Grid can be Cut into Sections
fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    fn count_groups(mut coords: Vec<(i32, i32)>) -> bool {
        coords.sort_unstable();
        let mut lines = 0;
        let mut overlap = 0;
        for (_, marker) in coords {
            overlap += marker;
            if overlap == 0 {
                lines += 1;
            }
        }
        lines >= 3
    }
    let mut xs = Vec::with_capacity(rectangles.len() * 2);
    let mut ys = Vec::with_capacity(rectangles.len() * 2);
    for r in &rectangles {
        xs.push((r[0], 1));
        xs.push((r[2], -1));
        ys.push((r[1], 1));
        ys.push((r[3], -1));
    }
    count_groups(xs) || count_groups(ys)
}

fn main() {
    println!(
        "{}",
        check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::check_valid_cuts;

    #[test]
    fn example1() {
        assert!(check_valid_cuts(
            5,
            vec![
                vec![1, 0, 5, 2],
                vec![0, 2, 2, 4],
                vec![3, 2, 5, 3],
                vec![0, 4, 4, 5]
            ]
        ));
    }

    #[test]
    fn example2() {
        assert!(check_valid_cuts(
            4,
            vec![
                vec![0, 0, 1, 1],
                vec![2, 0, 3, 4],
                vec![0, 2, 2, 3],
                vec![3, 0, 4, 3]
            ]
        ));
    }

    #[test]
    fn example3() {
        assert!(!check_valid_cuts(
            4,
            vec![
                vec![0, 2, 2, 4],
                vec![1, 0, 3, 2],
                vec![2, 2, 3, 4],
                vec![3, 0, 4, 2],
                vec![3, 2, 4, 4]
            ]
        ));
    }
}
