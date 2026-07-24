/// LeetCode #2634 - Filter Elements from Array (JS problem; Rust closure analogue)
fn filter(arr: Vec<i32>, fn_: impl Fn(i32, usize) -> bool) -> Vec<i32> {
    let mut ans = Vec::new();
    for (i, &x) in arr.iter().enumerate() {
        if fn_(x, i) {
            ans.push(x);
        }
    }
    ans
}

fn main() {
    println!("{:?}", filter(vec![0, 10, 20, 30], |n, _| n > 10));
}

#[cfg(test)]
mod tests {
    use super::filter;

    #[test]
    fn example_one() {
        assert_eq!(filter(vec![0, 10, 20, 30], |n, _| n > 10), vec![20, 30]);
    }

    #[test]
    fn example_two() {
        assert_eq!(filter(vec![1, 2, 3], |_, i| i == 0), vec![1]);
    }

    #[test]
    fn example_three() {
        // truthy if n + 1 != 0 (mirrors JS Boolean(n + 1))
        assert_eq!(
            filter(vec![-2, -1, 0, 1, 2], |n, _| n + 1 != 0),
            vec![-2, 0, 1, 2]
        );
    }
}
