/// LeetCode #2601 - Prime Subtraction Operation
fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
    let mut p = Vec::new();
    for i in 2..=1000 {
        let mut ok = true;
        for &j in &p {
            if i % j == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            p.push(i);
        }
    }

    let n = nums.len();
    for i in (0..n - 1).rev() {
        if nums[i] < nums[i + 1] {
            continue;
        }
        let target = nums[i] - nums[i + 1];
        let j = p.partition_point(|&x| x <= target);
        if j == p.len() || p[j] >= nums[i] {
            return false;
        }
        nums[i] -= p[j];
    }
    true
}

fn main() {
    println!("{}", prime_sub_operation(vec![4, 9, 6, 10]));
}

#[cfg(test)]
mod tests {
    use super::prime_sub_operation;

    #[test]
    fn example_one() {
        assert!(prime_sub_operation(vec![4, 9, 6, 10]));
    }

    #[test]
    fn example_two() {
        assert!(prime_sub_operation(vec![6, 8, 11, 12]));
    }

    #[test]
    fn example_three() {
        assert!(!prime_sub_operation(vec![5, 8, 3]));
    }
}
