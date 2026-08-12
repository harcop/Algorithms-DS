/// LeetCode #3171 - Find Subarray With Bitwise OR Closest to K
fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mx = *nums.iter().max().unwrap();
    let m = ((32 - mx.leading_zeros()) as usize).max(1);
    let mut cnt = vec![0i32; m];
    let mut s = 0i32;
    let mut i = 0usize;
    let mut ans = i32::MAX;
    for (j, &x) in nums.iter().enumerate() {
        s |= x;
        ans = ans.min((s - k).abs());
        for h in 0..m {
            if (x >> h) & 1 == 1 {
                cnt[h] += 1;
            }
        }
        while i < j && s > k {
            let y = nums[i];
            for h in 0..m {
                if (y >> h) & 1 == 1 {
                    cnt[h] -= 1;
                    if cnt[h] == 0 {
                        s ^= 1 << h;
                    }
                }
            }
            i += 1;
            ans = ans.min((s - k).abs());
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_difference(vec![1, 2, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::minimum_difference;

    #[test]
    fn example1() {
        assert_eq!(minimum_difference(vec![1, 2, 4, 5], 3), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_difference(vec![1, 3, 1, 3], 2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_difference(vec![1], 10), 9);
    }
}
