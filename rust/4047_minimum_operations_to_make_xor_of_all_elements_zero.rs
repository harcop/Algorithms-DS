/// LeetCode #4047 - Minimum Operations to Make XOR of All Elements Zero
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut xor_all = 0i32;
    let mut seen = [false; 2001];
    for &x in &nums {
        xor_all ^= x;
        seen[x as usize] = true;
    }
    if xor_all == 0 {
        return 0;
    }
    let vals: Vec<usize> = (1..=2000).filter(|&v| seen[v]).collect();
    if vals.len() == 1 {
        return -1;
    }
    const INF: i32 = 1_000_000;
    let mut dp = vec![INF; 2048];
    dp[0] = 0;
    for v in vals {
        let mut next = dp.clone();
        for x in 0..2048 {
            if dp[x] < INF {
                let y = x ^ v;
                next[y] = next[y].min(dp[x] + 1);
            }
        }
        dp = next;
    }
    let k = dp[xor_all as usize];
    if k as usize == nums.len() { -1 } else { k }
}

fn main() {
    println!("{}", min_operations(vec![8, 1, 4, 8, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![8, 1, 4, 8, 2]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 2, 3]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![1, 2, 4]), -1);
    }
}
