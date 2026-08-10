/// LeetCode #3116 - Kth Smallest Amount With Single Denomination Combination
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
    let n = coins.len();
    let k = k as i64;
    let check = |mx: i64| -> bool {
        let mut cnt = 0i64;
        for i in 1..(1usize << n) {
            let mut v = 1i64;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    v = lcm(v, coins[j] as i64);
                    if v > mx {
                        break;
                    }
                }
            }
            let m = i.count_ones();
            if m % 2 == 1 {
                cnt += mx / v;
            } else {
                cnt -= mx / v;
            }
        }
        cnt >= k
    };

    let mut l = 1i64;
    let mut r = 100_000_000_000i64;
    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", find_kth_smallest(vec![3, 6, 9], 3));
}

#[cfg(test)]
mod tests {
    use super::find_kth_smallest;

    #[test]
    fn example1() {
        assert_eq!(find_kth_smallest(vec![3, 6, 9], 3), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(find_kth_smallest(vec![5, 2], 7), 12);
    }
}
