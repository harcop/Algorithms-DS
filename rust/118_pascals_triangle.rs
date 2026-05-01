/// LeetCode #118 - Pascal's Triangle
fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let n = num_rows as usize;
    let mut out: Vec<Vec<i32>> = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = vec![1; i + 1];
        for j in 1..i {
            row[j] = out[i - 1][j - 1] + out[i - 1][j];
        }
        out.push(row);
    }
    out
}

fn main() {
    println!("{:?}", generate(5));
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    fn example_one() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(generate(1), vec![vec![1]]);
    }
}
