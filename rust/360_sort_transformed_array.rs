/// LeetCode #360 - Sort Transformed Array
fn sort_transformed(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut y: Vec<i32> = nums
        .into_iter()
        .map(|x| a * x * x + b * x + c)
        .collect();
    y.sort_unstable();
    y
}

fn main() {
    println!("{:?}", sort_transformed(vec![-4, -2, 2, 4], 1, 3, 5));
}

#[cfg(test)]
mod tests {
    use super::sort_transformed;

    #[test]
    fn quad() {
        assert_eq!(
            sort_transformed(vec![-4, -2, 2, 4], 1, 3, 5),
            vec![3, 9, 15, 33]
        );
    }

    #[test]
    fn linear() {
        assert_eq!(sort_transformed(vec![0, 1], 0, 3, 10), vec![10, 13]);
    }
}
