/// LeetCode #1552 - Magnetic Force Between Two Balls
fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut pos = position;
    pos.sort_unstable();
    let n = pos.len();
    let mut lo = 1i32;
    let mut hi = pos[n - 1] - pos[0];
    let mut ans = 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mut cnt = 1;
        let mut last = pos[0];
        for &p in pos.iter().skip(1) {
            if p - last >= mid {
                cnt += 1;
                last = p;
            }
        }
        if cnt >= m {
            ans = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_distance(vec![1, 2, 3, 4, 7], 3));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example_one() {
        assert_eq!(max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_distance(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3);
    }
}
