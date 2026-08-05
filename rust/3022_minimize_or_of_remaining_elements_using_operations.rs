/// LeetCode #3022 - Minimize OR of Remaining Elements Using Operations
fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i32;
    let mut ans = 0i32;
    let mut rans = 0i32;

    for i in (0..=29).rev() {
        let test = ans + (1 << i);
        let mut cnt = 0;
        let mut val = 0;
        for &num in &nums {
            if val == 0 {
                val = test & num;
            } else {
                val &= test & num;
            }
            if val != 0 {
                cnt += 1;
            }
        }
        if cnt > k {
            rans += 1 << i;
        } else {
            ans += 1 << i;
        }
    }

    rans
}

fn main() {
    println!("{}", min_or_after_operations(vec![3, 5, 3, 2, 7], 2));
    println!("{}", min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4));
    println!(
        "{}",
        min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::min_or_after_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_or_after_operations(vec![3, 5, 3, 2, 7], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1),
            15
        );
    }
}
