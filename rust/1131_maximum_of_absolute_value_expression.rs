/// LeetCode #1131 - Maximum of Absolute Value Expression
fn max_abs_val_expr(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0i32;
    for i in 0..n {
        for j in 0..n {
            let v = (arr[i] - arr[j]).abs()
                + (i as i32 - j as i32).abs()
                + (arr[i] + arr[j]).abs();
            ans = ans.max(v);
        }
    }
    ans
}

fn main() {
    println!("{}", max_abs_val_expr(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_abs_val_expr;

    #[test]
    fn example_one() {
        assert_eq!(max_abs_val_expr(vec![1, 2, 3, 4]), 11);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_abs_val_expr(vec![1, -1]), 3);
    }
}
