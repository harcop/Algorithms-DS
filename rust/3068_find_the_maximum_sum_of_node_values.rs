/// LeetCode #3068 - Find the Maximum Sum of Node Values
fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
    let mut f0 = 0i64;
    let mut f1 = i64::MIN / 4;

    for x in nums {
        let x = x as i64;
        let xored = (x as i32 ^ k) as i64;
        let new_f0 = std::cmp::max(f0 + x, f1 + xored);
        let new_f1 = std::cmp::max(f1 + x, f0 + xored);
        f0 = new_f0;
        f1 = new_f1;
    }

    f0
}

fn main() {
    println!(
        "{}",
        maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_value_sum;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_value_sum(vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]]),
            6
        );
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_value_sum(vec![2, 3], 7, vec![vec![0, 1]]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(
            maximum_value_sum(vec![7, 7, 7, 7, 7, 7], 3, vec![]),
            42
        );
    }
}
