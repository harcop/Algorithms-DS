/// LeetCode #1011 - Capacity To Ship Packages Within D Days
fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut lo = *weights.iter().max().unwrap();
    let mut hi: i32 = weights.iter().map(|&w| w).sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let mut need = 1i32;
        let mut cur = 0i32;
        for &w in &weights {
            if cur + w > mid {
                need += 1;
                cur = 0;
            }
            cur += w;
        }
        if need <= days {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5));
}

#[cfg(test)]
mod tests {
    use super::ship_within_days;

    #[test]
    fn example_one() {
        assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    }
}
