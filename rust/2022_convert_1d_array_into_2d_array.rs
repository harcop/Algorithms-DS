/// LeetCode #2022 - Convert 1D Array Into 2D Array
fn construct2_darray(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let (m, n) = (m as usize, n as usize);
    if m * n != original.len() {
        return Vec::new();
    }
    original
        .chunks(n)
        .map(|row| row.to_vec())
        .collect()
}

fn main() {
    println!("{:?}", construct2_darray(vec![1, 2, 3, 4], 2, 2));
}

#[cfg(test)]
mod tests {
    use super::construct2_darray;

    #[test]
    fn example_one() {
        assert_eq!(
            construct2_darray(vec![1, 2, 3, 4], 2, 2),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(construct2_darray(vec![1, 2, 3], 1, 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn example_three() {
        assert_eq!(construct2_darray(vec![1, 2], 1, 1), Vec::<Vec<i32>>::new());
    }
}
