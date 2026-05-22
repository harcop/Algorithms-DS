/// LeetCode #1222 - Queens That Can Attack the King
fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set = std::collections::HashSet::new();
    for q in &queens {
        set.insert((q[0], q[1]));
    }
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut ans = Vec::new();
    let (kx, ky) = (king[0], king[1]);
    for (dx, dy) in dirs {
        let mut x = kx;
        let mut y = ky;
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= 8 || y >= 8 {
                break;
            }
            if set.contains(&(x, y)) {
                ans.push(vec![x, y]);
                break;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        queens_attackthe_king(vec![vec![0, 1], vec![1, 0], vec![4, 0], vec![0, 4], vec![3, 3], vec![2, 4]], vec![0, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::queens_attackthe_king;

    fn mut_sort(v: &mut Vec<Vec<i32>>) {
        v.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    }

    #[test]
    fn example_one() {
        let mut got = queens_attackthe_king(
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4],
            ],
            vec![0, 0],
        );
        mut_sort(&mut got);
        let mut exp = vec![vec![0, 1], vec![1, 0], vec![3, 3]];
        mut_sort(&mut exp);
        assert_eq!(got, exp);
    }

    #[test]
    fn example_two() {
        let mut got = queens_attackthe_king(
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![3, 5],
                vec![4, 4],
                vec![4, 5],
            ],
            vec![3, 3],
        );
        mut_sort(&mut got);
        let mut exp = vec![vec![2, 2], vec![3, 4], vec![4, 4]];
        mut_sort(&mut exp);
        assert_eq!(got, exp);
    }
}
