/// LeetCode #4051 - Count Subarrays with Distant Sums
fn distant_subarrays(nums: Vec<i32>, goal: i32, k: i32) -> i64 {
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let mut order = s.clone();
    order.sort_unstable();
    let mut bit = vec![0i64; order.len() + 2];
    let mut ans = n as i64 * (n as i64 + 1) / 2;
    let goal = goal as i64;
    let k = k as i64;
    for &v in &s {
        let a = v - goal - k + 1;
        let b = v - goal + k - 1;
        let l = order.partition_point(|&x| x < a) + 1;
        let r = order.partition_point(|&x| x < b + 1);
        if l <= r {
            ans -= query(&bit, r) - query(&bit, l - 1);
        }
        update(&mut bit, order.partition_point(|&x| x < v) + 1);
    }
    ans
}

fn update(bit: &mut [i64], mut x: usize) {
    while x < bit.len() {
        bit[x] += 1;
        x += x & x.wrapping_neg();
    }
}

fn query(bit: &[i64], mut x: usize) -> i64 {
    let mut sum = 0i64;
    while x > 0 {
        sum += bit[x];
        x -= x & x.wrapping_neg();
    }
    sum
}

fn main() {
    println!("{}", distant_subarrays(vec![1, 2, 1], 4, 1));
}

#[cfg(test)]
mod tests {
    use super::distant_subarrays;

    #[test]
    fn example1() {
        assert_eq!(distant_subarrays(vec![1, 2, 1], 4, 1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(distant_subarrays(vec![2, -1, 3], 2, 2), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(distant_subarrays(vec![-3, 1, 2], 0, 3), 2);
    }
}
