/// LeetCode #327 - Count of Range Sum (merge-sort counting on prefix sums)
fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut pref: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + nums[i] as i64;
    }
    let mut tmp = vec![0i64; n + 1];
    let lower = lower as i64;
    let upper = upper as i64;

    fn dac(
        pref: &mut [i64],
        tmp: &mut [i64],
        lo: usize,
        hi: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if lo == hi {
            return 0;
        }
        let mid = (lo + hi) / 2;
        let mut count =
            dac(pref, tmp, lo, mid, lower, upper) + dac(pref, tmp, mid + 1, hi, lower, upper);
        let mut j = mid + 1;
        let mut k = mid + 1;
        for i in lo..=mid {
            while j <= hi && pref[j] - pref[i] < lower {
                j += 1;
            }
            while k <= hi && pref[k] - pref[i] <= upper {
                k += 1;
            }
            count += (k - j) as i32;
        }
        let mut p = lo;
        let mut q = mid + 1;
        let mut t = lo;
        while p <= mid && q <= hi {
            if pref[p] <= pref[q] {
                tmp[t] = pref[p];
                p += 1;
            } else {
                tmp[t] = pref[q];
                q += 1;
            }
            t += 1;
        }
        while p <= mid {
            tmp[t] = pref[p];
            p += 1;
            t += 1;
        }
        while q <= hi {
            tmp[t] = pref[q];
            q += 1;
            t += 1;
        }
        pref[lo..=hi].copy_from_slice(&tmp[lo..=hi]);
        count
    }

    dac(&mut pref, &mut tmp, 0, n, lower, upper)
}

fn main() {
    println!("{}", count_range_sum(vec![-2, 5, -1], -2, 2));
}

#[cfg(test)]
mod tests {
    use super::count_range_sum;

    #[test]
    fn example() {
        assert_eq!(count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    }
}
