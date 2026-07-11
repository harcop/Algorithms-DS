/// LeetCode #2358 - Maximum Number of Groups Entering a Competition
fn maximum_groups(grades: Vec<i32>) -> i32 {
    let n = grades.len() as i64;
    let (mut l, mut r) = (0i64, n);
    while l < r {
        let mid = (l + r + 1) / 2;
        if mid * mid + mid > 2 * n {
            r = mid - 1;
        } else {
            l = mid;
        }
    }
    l as i32
}

fn main() {
    println!("{}", maximum_groups(vec![10, 6, 12, 7, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_groups;

    #[test]
    fn example_one() {
        assert_eq!(maximum_groups(vec![10, 6, 12, 7, 3, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_groups(vec![8, 8]), 1);
    }
}
