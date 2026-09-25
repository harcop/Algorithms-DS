/// LeetCode #3937 - Minimum Operations to Make Array Modulo Alternating I
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let rem: Vec<i32> = nums.iter().map(|v| v % k).collect();
    let mut ans = i32::MAX;
    for x in 0..k {
        for y in 0..k {
            if x == y {
                continue;
            }
            let mut cnt = 0i32;
            for (i, &v) in rem.iter().enumerate() {
                let target = if i % 2 == 0 { x } else { y };
                let diff = (target - v).abs();
                cnt += diff.min(k - diff);
            }
            ans = ans.min(cnt);
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![1, 4, 2, 8], 3));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 4, 2, 8], 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 1, 1], 3), 1);
    }
}
