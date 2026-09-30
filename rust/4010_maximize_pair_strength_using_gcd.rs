/// LeetCode #4010 - Maximize Pair Strength Using GCD
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn max_pair_strength(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut ans = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            let g = gcd(nums[i] as i64, nums[j] as i64);
            let strength = (nums[i] as i64 / g) * (nums[j] as i64 / g);
            ans = ans.max(strength);
        }
    }
    ans
}

fn main() {
    println!("{}", max_pair_strength(vec![2, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_pair_strength;

    #[test]
    fn example1() {
        assert_eq!(max_pair_strength(vec![2, 3, 5]), 15);
    }

    #[test]
    fn example2() {
        assert_eq!(max_pair_strength(vec![4, 6, 8]), 12);
    }

    #[test]
    fn example3() {
        assert_eq!(max_pair_strength(vec![3, 3]), 1);
    }
}
