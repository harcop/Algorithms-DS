/// LeetCode #3417 - Zigzag Grid Traversal With Skip
fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ok = true;
    let mut ans = Vec::new();
    for (i, mut row) in grid.into_iter().enumerate() {
        if i % 2 == 1 {
            row.reverse();
        }
        for x in row {
            if ok {
                ans.push(x);
            }
            ok = !ok;
        }
    }
    ans
}

fn main() {
    println!("{:?}", zigzag_traversal(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::zigzag_traversal;

    #[test]
    fn example1() {
        assert_eq!(zigzag_traversal(vec![vec![1, 2], vec![3, 4]]), vec![1, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            zigzag_traversal(vec![vec![2, 1], vec![2, 1], vec![2, 1]]),
            vec![2, 1, 2]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            zigzag_traversal(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 3, 5, 7, 9]
        );
    }
}
