/// LeetCode #3699 - Number of ZigZag Arrays I
fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let m = (r - l + 1) as usize;
    let mut up = vec![1i32; m];
    let mut down = vec![1i32; m];
    for _ in 0..n - 1 {
        let mut pre = vec![0i32; m + 1];
        let mut suf = vec![0i32; m + 1];
        for i in 0..m {
            pre[i + 1] = (pre[i] + down[i]) % MOD;
        }
        for i in (0..m).rev() {
            suf[i] = (suf[i + 1] + up[i]) % MOD;
        }
        up = pre[..m].to_vec();
        down = suf[1..].to_vec();
    }
    let mut ans = 0i32;
    for &x in up.iter().chain(down.iter()) {
        ans = (ans + x) % MOD;
    }
    ans
}

fn main() {
    println!("{}", zig_zag_arrays(3, 4, 5));
}

#[cfg(test)]
mod tests {
    use super::zig_zag_arrays;

    #[test]
    fn example1() {
        assert_eq!(zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(zig_zag_arrays(3, 1, 3), 10);
    }
}
