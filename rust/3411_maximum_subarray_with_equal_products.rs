/// LeetCode #3411 - Maximum Subarray With Equal Products
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let max_p = nums.iter().fold(1i64, |l, &x| lcm(l, x as i64)) * *nums.iter().max().unwrap() as i64;
    let mut ans = 0;
    for i in 0..n {
        let mut p = 1i64;
        let mut g = 0i64;
        let mut l = 1i64;
        for j in i..n {
            p *= nums[j] as i64;
            g = gcd(g, nums[j] as i64);
            l = lcm(l, nums[j] as i64);
            if p == g * l {
                ans = ans.max(j - i + 1);
            }
            if p > max_p {
                break;
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", max_length(vec![1, 2, 1, 2, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_length;

    #[test]
    fn example1() {
        assert_eq!(max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(max_length(vec![2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(max_length(vec![1, 2, 3, 1, 4, 5, 1]), 5);
    }
}
