/// LeetCode #2528 - Maximize the Minimum Powered City
fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let r = r as usize;
    let mut left = *stations.iter().min().unwrap() as i64;
    let mut right = stations.iter().map(|&x| x as i64).sum::<i64>() + k as i64 + 1;

    while left < right {
        let mid = (left + right) / 2;
        if check(&stations, r, k, mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

fn check(stations: &[i32], r: usize, k: i32, min_power: i64) -> bool {
    let n = stations.len();
    let mut s = stations.to_vec();
    let mut additional = k as i64;
    let mut power: i64 = s[..r.min(n)].iter().map(|&x| x as i64).sum();

    for i in 0..n {
        if i + r < n {
            power += s[i + r] as i64;
        }
        if power < min_power {
            let required = min_power - power;
            if required > additional {
                return false;
            }
            s[(i + r).min(n - 1)] += required as i32;
            additional -= required;
            power += required;
        }
        if i >= r {
            power -= s[i - r] as i64;
        }
    }
    true
}

fn main() {
    println!("{}", max_power(vec![1, 2, 1, 2], 1, 2));
}

#[cfg(test)]
mod tests {
    use super::max_power;

    #[test]
    fn example_one() {
        assert_eq!(max_power(vec![1, 2, 1, 2], 1, 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_power(vec![4, 4, 4, 4], 0, 3), 4);
    }
}
