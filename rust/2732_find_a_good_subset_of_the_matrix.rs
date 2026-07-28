/// LeetCode #2732 - Find a Good Subset of the Matrix
use std::collections::HashMap;

fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut g: HashMap<i32, i32> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        let mut mask = 0;
        for (j, &x) in row.iter().enumerate() {
            mask |= x << j;
        }
        if mask == 0 {
            return vec![i as i32];
        }
        g.insert(mask, i as i32);
    }
    for (&a, &i) in g.iter() {
        for (&b, &j) in g.iter() {
            if a & b == 0 {
                return vec![i.min(j), i.max(j)];
            }
        }
    }
    vec![]
}

fn main() {
    println!(
        "{:?}",
        good_subsetof_binary_matrix(vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 1, 1, 1]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::good_subsetof_binary_matrix;

    #[test]
    fn example_one() {
        assert_eq!(
            good_subsetof_binary_matrix(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 1, 1, 1]
            ]),
            vec![0, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(good_subsetof_binary_matrix(vec![vec![0]]), vec![0]);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            good_subsetof_binary_matrix(vec![vec![1, 1, 1], vec![1, 1, 1]]),
            vec![]
        );
    }
}
