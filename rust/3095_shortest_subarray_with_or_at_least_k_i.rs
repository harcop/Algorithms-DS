/// LeetCode #3095 - Shortest Subarray With OR at Least K I
fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut cnt = [0i32; 32];
    let mut ans = n as i32 + 1;
    let mut s = 0;
    let mut i = 0usize;

    for (j, &x) in nums.iter().enumerate() {
        s |= x;
        for h in 0..32 {
            if (x >> h) & 1 == 1 {
                cnt[h] += 1;
            }
        }
        while s >= k && i <= j {
            ans = ans.min((j - i + 1) as i32);
            let y = nums[i];
            for h in 0..32 {
                if (y >> h) & 1 == 1 {
                    cnt[h] -= 1;
                    if cnt[h] == 0 {
                        s ^= 1 << h;
                    }
                }
            }
            i += 1;
        }
    }
    if ans > n as i32 {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", minimum_subarray_length(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_subarray_length;

    #[test]
    fn example1() {
        assert_eq!(minimum_subarray_length(vec![1, 2, 3], 2), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_subarray_length(vec![2, 1, 8], 10), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_subarray_length(vec![1, 2], 0), 1);
    }
}
