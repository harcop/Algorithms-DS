/// LeetCode #1299 - Replace Elements with Greatest Element on Right Side
fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }
    let mut ans = vec![-1; n];
    let mut mx = arr[n - 1];
    for i in (0..n - 1).rev() {
        ans[i] = mx;
        mx = mx.max(arr[i]);
    }
    ans
}

fn main() {
    println!("{:?}", replace_elements(vec![17, 18, 5, 4, 6, 1]));
}

#[cfg(test)]
mod tests {
    use super::replace_elements;

    #[test]
    fn example_one() {
        assert_eq!(
            replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(replace_elements(vec![400]), vec![-1]);
    }
}
