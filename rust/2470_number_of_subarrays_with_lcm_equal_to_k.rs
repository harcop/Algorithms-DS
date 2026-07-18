/// LeetCode #2470 - Number of Subarrays With LCM Equal to K
fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = a % b;
            a = b;
            b = temp;
        }
        a
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / gcd(a, b) * b
    }

    let k = k as i64;
    let mut answer = 0;

    for i in 0..nums.len() {
        let mut running = nums[i] as i64;
        for j in i..nums.len() {
            running = lcm(running, nums[j] as i64);
            if running > k {
                break;
            }
            if running == k {
                answer += 1;
            }
        }
    }

    answer
}

fn main() {
    println!("{}", subarray_lcm(vec![3, 6, 2, 7, 1], 6));
}

#[cfg(test)]
mod tests {
    use super::subarray_lcm;

    #[test]
    fn example_one() {
        assert_eq!(subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(subarray_lcm(vec![3], 2), 0);
    }
}
