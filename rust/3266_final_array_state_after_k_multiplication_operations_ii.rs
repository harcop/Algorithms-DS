/// LeetCode #3266 - Final Array State After K Multiplication Operations II
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn qpow(mut a: i64, mut n: i64, modulus: i64) -> i64 {
    let mut ans = 1i64 % modulus;
    a %= modulus;
    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % modulus;
        }
        a = a * a % modulus;
        n >>= 1;
    }
    ans
}

fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
    if multiplier == 1 {
        return nums;
    }
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let m = *nums.iter().max().unwrap() as i64;
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    for (i, &x) in nums.iter().enumerate() {
        pq.push(Reverse((x as i64, i)));
    }
    while k > 0 && pq.peek().map(|Reverse((x, _))| *x).unwrap() < m {
        let Reverse((x, i)) = pq.pop().unwrap();
        pq.push(Reverse((x * multiplier as i64, i)));
        k -= 1;
    }
    let mut items = Vec::with_capacity(n);
    while let Some(Reverse(p)) = pq.pop() {
        items.push(p);
    }
    items.sort_unstable();
    let kn = k as i64;
    let nn = n as i64;
    for (i, (x, j)) in items.into_iter().enumerate() {
        let power = kn / nn + if (i as i64) < kn % nn { 1 } else { 0 };
        nums[j] = ((x % MOD) * qpow(multiplier as i64, power, MOD) % MOD) as i32;
    }
    nums
}

fn main() {
    println!("{:?}", get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
}

#[cfg(test)]
mod tests {
    use super::get_final_state;

    #[test]
    fn example1() {
        assert_eq!(get_final_state(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            get_final_state(vec![100000, 2000], 2, 1000000),
            vec![999999307, 999999993]
        );
    }
}
