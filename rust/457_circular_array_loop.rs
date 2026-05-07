/// LeetCode #457 - Circular Array Loop (non-zero length, same-direction cycle)
fn circular_array_loop(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }

    fn next_idx(nums: &[i32], i: usize) -> usize {
        let n = nums.len() as i32;
        (((i as i32 + nums[i]) % n + n) % n) as usize
    }

    for s in 0..n {
        if nums[s] == 0 {
            continue;
        }
        let mut slow = s;
        let mut fast = s;
        loop {
            slow = next_idx(&nums, slow);
            fast = next_idx(&nums, fast);
            fast = next_idx(&nums, fast);
            if slow == fast {
                break;
            }
        }
        if slow == next_idx(&nums, slow) {
            continue;
        }
        let mut p = s;
        let sign = nums[s] > 0;
        let mut ok = true;
        while {
            if (nums[p] > 0) != sign {
                ok = false;
            }
            p = next_idx(&nums, p);
            p != s
        } {}
        if ok {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", circular_array_loop(vec![2, -1, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert!(circular_array_loop(vec![2, -1, 1, 2, 2]));
        assert!(!circular_array_loop(vec![-1, 2]));
    }
}
