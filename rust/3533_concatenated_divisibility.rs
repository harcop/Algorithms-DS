/// LeetCode #3533 - Concatenated Divisibility
fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    nums.sort_unstable();
    let mut pows = vec![0i32; n];
    for i in 0..n {
        let mut p = 1i32;
        let mut x = nums[i];
        while x > 0 {
            p = p * 10 % k;
            x /= 10;
        }
        pows[i] = p;
    }
    let full = (1usize << n) - 1;
    let mut mem = vec![vec![-1i8; k as usize]; 1 << n];
    fn can(
        mask: usize,
        rem: i32,
        nums: &[i32],
        pows: &[i32],
        k: i32,
        full: usize,
        mem: &mut [Vec<i8>],
    ) -> bool {
        if mem[mask][rem as usize] != -1 {
            return mem[mask][rem as usize] == 1;
        }
        if mask == full {
            let ok = rem == 0;
            mem[mask][rem as usize] = ok as i8;
            return ok;
        }
        for i in 0..nums.len() {
            if (mask >> i) & 1 == 1 {
                continue;
            }
            let new_rem = (rem * pows[i] + nums[i]) % k;
            if can(mask | (1 << i), new_rem, nums, pows, k, full, mem) {
                mem[mask][rem as usize] = 1;
                return true;
            }
        }
        mem[mask][rem as usize] = 0;
        false
    }
    if !can(0, 0, &nums, &pows, k, full, &mut mem) {
        return vec![];
    }
    let mut ans = Vec::new();
    let mut mask = 0usize;
    let mut rem = 0i32;
    while mask != full {
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                continue;
            }
            let new_rem = (rem * pows[i] + nums[i]) % k;
            let new_mask = mask | (1 << i);
            if can(new_mask, new_rem, &nums, &pows, k, full, &mut mem) {
                ans.push(nums[i]);
                mask = new_mask;
                rem = new_rem;
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", concatenated_divisibility(vec![3, 12, 45], 5));
}

#[cfg(test)]
mod tests {
    use super::concatenated_divisibility;

    #[test]
    fn example1() {
        assert_eq!(concatenated_divisibility(vec![3, 12, 45], 5), vec![3, 12, 45]);
    }

    #[test]
    fn example2() {
        assert_eq!(concatenated_divisibility(vec![10, 5], 10), vec![5, 10]);
    }

    #[test]
    fn example3() {
        assert_eq!(concatenated_divisibility(vec![1, 2, 3], 5), vec![] as Vec<i32>);
    }
}
