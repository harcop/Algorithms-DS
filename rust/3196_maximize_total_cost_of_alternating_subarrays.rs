/// LeetCode #3196 - Maximize Total Cost of Alternating Subarrays
fn maximum_total_cost(nums: Vec<i32>) -> i64 {
    let mut f0 = 0i64;
    let mut f1 = 0i64;
    for &x in nums.iter().rev() {
        let x = x as i64;
        let n0 = x + f1;
        let n1 = (x + f1).max(-x + f0);
        f0 = n0;
        f1 = n1;
    }
    f0
}

fn main() {
    println!("{}", maximum_total_cost(vec![1, -2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_total_cost;

    #[test]
    fn example1() {
        assert_eq!(maximum_total_cost(vec![1, -2, 3, 4]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_total_cost(vec![1, -1, 1, -1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_total_cost(vec![0]), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(maximum_total_cost(vec![1, -1]), 2);
    }
}
