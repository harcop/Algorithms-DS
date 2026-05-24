/// LeetCode #1338 - Maximum Candies Allocated to K Children
fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut lo = 0i64;
    let mut hi = *candies.iter().max().unwrap_or(&0) as i64;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if mid == 0 {
            break;
        }
        let mut cnt = 0i64;
        for &c in &candies {
            cnt += (c as i64) / mid;
            if cnt >= k {
                break;
            }
        }
        if cnt >= k {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", maximum_candies(vec![5, 8, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_candies;

    #[test]
    fn example_one() {
        assert_eq!(maximum_candies(vec![5, 8, 6], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_candies(vec![2, 5], 11), 0);
    }
}
