/// LeetCode #3653 - XOR After Range Multiplication Queries I
fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    for q in queries {
        let (l, r, k, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as i64);
        let mut idx = l;
        while idx <= r {
            nums[idx] = ((nums[idx] as i64 * v) % MOD) as i32;
            idx += k;
        }
    }
    nums.into_iter().fold(0, |a, x| a ^ x)
}

fn main() {
    println!("{}", xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::xor_after_queries;

    #[test]
    fn example1() {
        assert_eq!(xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            xor_after_queries(vec![2, 3, 1, 5, 4], vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]),
            31
        );
    }
}
