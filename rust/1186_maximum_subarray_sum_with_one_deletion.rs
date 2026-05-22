/// LeetCode #1186 - Maximum Subarray Sum with One Deletion
fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut best = arr[0];
    let mut no_del = arr[0];
    let mut with_del = 0;
    for &x in arr.iter().skip(1) {
        with_del = (with_del + x).max(no_del);
        no_del = (no_del + x).max(x);
        best = best.max(no_del).max(with_del);
    }
    best
}

fn main() {
    println!("{}", maximum_sum(vec![1, -2, 0, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_sum(vec![1, -2, 0, 3]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_sum(vec![1, -2, -2, 3]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_sum(vec![-1, -1, -1, -1]), -1);
    }
}
