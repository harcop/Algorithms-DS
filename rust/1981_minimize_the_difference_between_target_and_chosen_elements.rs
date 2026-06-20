/// LeetCode #1981 - Minimize the Difference Between Target and Chosen Elements
use std::collections::HashSet;

fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut f: HashSet<i32> = HashSet::from([0]);
    for row in mat {
        let mut nf = HashSet::new();
        for a in &f {
            for b in &row {
                nf.insert(a + b);
            }
        }
        f = nf;
    }
    f.iter().map(|&v| (v - target).abs()).min().unwrap()
}

fn main() {
    println!(
        "{}",
        minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13)
    );
}

#[cfg(test)]
mod tests {
    use super::minimize_the_difference;

    #[test]
    fn example_one() {
        assert_eq!(
            minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13),
            0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100), 94);
    }
}
