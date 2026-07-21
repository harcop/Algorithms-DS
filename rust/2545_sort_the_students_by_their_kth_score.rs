/// LeetCode #2545 - Sort the Students by Their Kth Score
fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    score.sort_by(|a, b| b[k].cmp(&a[k]));
    score
}

fn main() {
    let score = vec![
        vec![10, 6, 9, 1],
        vec![7, 5, 11, 2],
        vec![4, 8, 3, 15],
    ];
    println!("{:?}", sort_the_students(score, 2));
}

#[cfg(test)]
mod tests {
    use super::sort_the_students;

    #[test]
    fn example_one() {
        let score = vec![
            vec![10, 6, 9, 1],
            vec![7, 5, 11, 2],
            vec![4, 8, 3, 15],
        ];
        let expected = vec![
            vec![7, 5, 11, 2],
            vec![10, 6, 9, 1],
            vec![4, 8, 3, 15],
        ];
        assert_eq!(sort_the_students(score, 2), expected);
    }

    #[test]
    fn example_two() {
        let score = vec![vec![3, 4], vec![5, 6], vec![2, 2]];
        let expected = vec![vec![5, 6], vec![3, 4], vec![2, 2]];
        assert_eq!(sort_the_students(score, 0), expected);
    }
}
