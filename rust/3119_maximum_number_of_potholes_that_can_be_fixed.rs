/// LeetCode #3119 - Maximum Number of Potholes That Can Be Fixed
fn max_potholes(road: String, mut budget: i32) -> i32 {
    let mut cs: Vec<u8> = road.into_bytes();
    cs.push(b'.');
    let n = cs.len();
    let mut cnt = vec![0i32; n];
    let mut k = 0usize;
    for &c in &cs {
        if c == b'x' {
            k += 1;
        } else if k > 0 {
            cnt[k] += 1;
            k = 0;
        }
    }
    let mut ans = 0;
    for k in (1..n).rev() {
        if budget == 0 {
            break;
        }
        let t = (budget / (k as i32 + 1)).min(cnt[k]);
        ans += t * k as i32;
        budget -= t * (k as i32 + 1);
        cnt[k - 1] += cnt[k] - t;
    }
    ans
}

fn main() {
    println!("{}", max_potholes("..xxxxx".into(), 4));
}

#[cfg(test)]
mod tests {
    use super::max_potholes;

    #[test]
    fn example1() {
        assert_eq!(max_potholes("..".into(), 5), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(max_potholes("..xxxxx".into(), 4), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(max_potholes("x.x.xxx...x".into(), 14), 6);
    }
}
