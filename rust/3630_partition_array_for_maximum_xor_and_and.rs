/// LeetCode #3630 - Partition Array for Maximum XOR and AND
fn max_xor_subset(vals: &[i32], bit_len: usize) -> i32 {
    let mut base = vec![0i32; bit_len];
    for &v in vals {
        let mut x = v;
        for i in (0..bit_len).rev() {
            if (x & (1 << i)) == 0 {
                continue;
            }
            if base[i] == 0 {
                base[i] = x;
                break;
            }
            x ^= base[i];
        }
    }
    let mut max_xor = 0i32;
    for i in (0..bit_len).rev() {
        if (max_xor ^ base[i]) > max_xor {
            max_xor ^= base[i];
        }
    }
    max_xor
}

fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mx = *nums.iter().max().unwrap_or(&1);
    let bit_len = (32 - mx.leading_zeros()) as usize;
    let full = (1usize << n) - 1;
    let mut and_arr = vec![0i32; 1 << n];
    let mut xor_arr = vec![0i32; 1 << n];
    for mask in 1usize..(1usize << n) {
        let lb = mask & (!mask + 1);
        let i = lb.trailing_zeros() as usize;
        and_arr[mask] = if mask ^ lb != 0 {
            and_arr[mask ^ lb] & nums[i]
        } else {
            nums[i]
        };
        xor_arr[mask] = xor_arr[mask ^ lb] ^ nums[i];
    }
    let mut result = 0i64;
    for mask in 0usize..(1usize << n) {
        let total_and = and_arr[mask] as i64;
        let remain = full ^ mask;
        let total_xor = xor_arr[remain] as i64;
        let mut vals = Vec::new();
        let mut r = remain;
        while r != 0 {
            let i = r.trailing_zeros() as usize;
            vals.push(nums[i] & !xor_arr[remain]);
            r &= r - 1;
        }
        let mx_xor = max_xor_subset(&vals, bit_len) as i64;
        result = result.max(total_and + total_xor + 2 * mx_xor);
    }
    result
}

fn main() {
    println!("{}", maximize_xor_and_xor(vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximize_xor_and_xor;

    #[test]
    fn example1() {
        assert_eq!(maximize_xor_and_xor(vec![2, 3]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(maximize_xor_and_xor(vec![1, 3, 2]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(maximize_xor_and_xor(vec![2, 3, 6, 7]), 15);
    }
}
