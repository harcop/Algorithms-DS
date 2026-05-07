/// LeetCode #330 - Patching Array
fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let n = n as i64;
    let mut patched = 0i32;
    let mut miss = 1i64;
    let mut i = 0usize;
    while miss <= n {
        if i < nums.len() && nums[i] as i64 <= miss {
            miss += nums[i] as i64;
            i += 1;
        } else {
            patched += 1;
            miss += miss;
        }
    }
    patched
}

fn main() {
    println!("{}", min_patches(vec![1, 5, 10], 20));
}

#[cfg(test)]
mod tests {
    use super::min_patches;

    #[test]
    fn examples() {
        assert_eq!(min_patches(vec![1, 3], 6), 1);
        assert_eq!(min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(min_patches(vec![1, 2, 2], 5), 0);
    }
}
