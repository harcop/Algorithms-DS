/// LeetCode #2344 - Minimum Deletions to Make Array Divisible
fn min_operations(mut nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
    let mut x = 0;
    for v in nums_divide {
        x = gcd(x, v);
    }
    nums.sort_unstable();
    for (i, &v) in nums.iter().enumerate() {
        if x % v == 0 {
            return i as i32;
        }
    }
    -1
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn main() {
    println!(
        "{}",
        min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15])
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![4, 3, 6], vec![8, 2, 6, 10]), -1);
    }
}
