/// LeetCode #3639 - Minimum Time to Activate String
fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
    let n = s.len();
    let mut left: Vec<i32> = (0..n as i32).map(|i| i - 1).collect();
    let mut right: Vec<i32> = (0..n as i32).map(|i| i + 1).collect();
    let mut cnt = (n as i64 + 1) * n as i64 / 2;
    let k = k as i64;
    if cnt < k {
        return -1;
    }
    let mut t = order.len() as i32 - 1;
    while t >= 0 {
        let i = order[t as usize] as usize;
        let l = left[i];
        let r = right[i];
        cnt -= (i as i64 - l as i64) * (r as i64 - i as i64);
        if cnt < k {
            break;
        }
        if l >= 0 {
            right[l as usize] = r;
        }
        if (r as usize) < n {
            left[r as usize] = l;
        }
        t -= 1;
    }
    t
}

fn main() {
    println!("{}", min_time("abc".into(), vec![1, 0, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    #[test]
    fn example1() {
        assert_eq!(min_time("abc".into(), vec![1, 0, 2], 2), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(min_time("cat".into(), vec![0, 2, 1], 6), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_time("xy".into(), vec![0, 1], 4), -1);
    }
}
