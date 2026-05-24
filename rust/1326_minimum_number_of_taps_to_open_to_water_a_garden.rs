/// LeetCode #1326 - Minimum Number of Taps to Open to Water a Garden
fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut reach = vec![0; n + 1];
    for (i, &r) in ranges.iter().enumerate() {
        let l = i.saturating_sub(r as usize);
        let ri = (i + r as usize).min(n);
        reach[l] = reach[l].max(ri);
    }
    let mut end = 0;
    let mut farthest = 0;
    let mut taps = 0;
    for i in 0..=n {
        farthest = farthest.max(reach[i]);
        if i == end {
            if farthest == end {
                return -1;
            }
            taps += 1;
            end = farthest;
            if end >= n {
                return taps;
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_taps(5, vec![3, 4, 1, 1, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_taps;

    #[test]
    fn example_one() {
        assert_eq!(min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_taps(3, vec![0, 0, 0, 0]), -1);
    }
}
