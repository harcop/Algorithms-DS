/// LeetCode #3768 - Minimum Inversion Count in Subarrays of Fixed Length
fn min_inversion_count(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut vals = nums.clone();
    vals.sort_unstable();
    vals.dedup();
    let m = vals.len();
    let rank = |x: i32| vals.partition_point(|&v| v < x);
    let mut bit = vec![0i64; m + 2];
    let add = |bit: &mut [i64], mut i: usize, v: i64| {
        i += 1;
        while i < bit.len() {
            bit[i] += v;
            i += i & i.wrapping_neg();
        }
    };
    let query = |bit: &[i64], mut i: isize| -> i64 {
        let mut s = 0i64;
        while i > 0 {
            s += bit[i as usize];
            i -= i & -i;
        }
        s
    };
    let mut cnt = 0i64;
    let mut ans = i64::MAX;
    for i in 0..n {
        let r = rank(nums[i]);
        cnt += query(&bit, m as isize) - query(&bit, r as isize);
        add(&mut bit, r, 1);
        if i + 1 >= k {
            ans = ans.min(cnt);
            let old = rank(nums[i + 1 - k]);
            add(&mut bit, old, -1);
            cnt -= query(&bit, old as isize);
        }
    }
    ans
}

fn main() {
    println!("{}", min_inversion_count(vec![3, 1, 2, 5, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::min_inversion_count;

    #[test]
    fn example1() {
        assert_eq!(min_inversion_count(vec![3, 1, 2, 5, 4], 3), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(min_inversion_count(vec![5, 3, 2, 1], 4), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_inversion_count(vec![2, 1], 1), 0);
    }
}
