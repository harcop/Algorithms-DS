/// LeetCode #3605 - Minimum Stability Factor of Array
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn min_stable(nums: Vec<i32>, max_c: i32) -> i32 {
    let n = nums.len();
    let non_one = nums.iter().filter(|&&x| x != 1).count() as i32;
    if non_one <= max_c {
        return 0;
    }
    let mut log = vec![0usize; n + 1];
    for i in 2..=n {
        log[i] = log[i / 2] + 1;
    }
    let kmax = log[n] + 1;
    let mut st = vec![vec![0i32; n]; kmax];
    st[0].clone_from_slice(&nums);
    for k in 1..kmax {
        let len = 1usize << k;
        let half = 1usize << (k - 1);
        if n < len {
            break;
        }
        for i in 0..=n - len {
            st[k][i] = gcd(st[k - 1][i], st[k - 1][i + half]);
        }
    }
    let query = |l: usize, r: usize| -> i32 {
        let k = log[r - l + 1];
        gcd(st[k][l], st[k][r + 1 - (1usize << k)])
    };
    let can = |limit: usize| -> bool {
        let mut used = 0i32;
        let mut i = 0usize;
        while i + limit < n {
            if query(i, i + limit) >= 2 {
                used += 1;
                if used > max_c {
                    return false;
                }
                i += limit + 1;
            } else {
                i += 1;
            }
        }
        true
    };
    let (mut l, mut r, mut ans) = (1usize, n - 1, n as i32);
    while l <= r {
        let mid = (l + r) / 2;
        if can(mid) {
            ans = mid as i32;
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_stable(vec![3, 5, 10], 1));
}

#[cfg(test)]
mod tests {
    use super::min_stable;

    #[test]
    fn example1() {
        assert_eq!(min_stable(vec![3, 5, 10], 1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_stable(vec![2, 6, 8], 2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_stable(vec![2, 4, 9, 6], 1), 2);
    }
}
