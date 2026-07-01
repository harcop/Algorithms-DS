/// LeetCode #2201 - Count Artifacts That Can Be Extracted
use std::collections::HashSet;

fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
    let n = n;
    let mut dug: HashSet<i32> = HashSet::new();
    for p in dig {
        dug.insert(p[0] * n + p[1]);
    }

    let check = |a: &Vec<i32>| -> i32 {
        let x1 = a[0];
        let y1 = a[1];
        let x2 = a[2];
        let y2 = a[3];
        for x in x1..=x2 {
            for y in y1..=y2 {
                if !dug.contains(&(x * n + y)) {
                    return 0;
                }
            }
        }
        1
    };

    artifacts.iter().map(check).sum()
}

fn main() {
    println!(
        "{}",
        dig_artifacts(
            2,
            vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
            vec![vec![0, 0], vec![0, 1]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::dig_artifacts;

    #[test]
    fn example_one() {
        assert_eq!(
            dig_artifacts(
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1]],
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            dig_artifacts(
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1], vec![1, 1]],
            ),
            2
        );
    }
}
