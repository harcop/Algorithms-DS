/// LeetCode #875 - Koko Eating Bananas
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi = *piles.iter().max().unwrap_or(&1);
    let h = h as i64;

    fn hours(piles: &[i32], k: i32) -> i64 {
        piles.iter().map(|&p| ((p as i64) + k as i64 - 1) / k as i64).sum()
    }

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if hours(&piles, mid) <= h {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", min_eating_speed(vec![3, 6, 7, 11], 8));
}

#[cfg(test)]
mod tests {
    use super::min_eating_speed;

    #[test]
    fn example_one() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }
}
