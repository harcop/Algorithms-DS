/// LeetCode #3464 - Maximize the Distance Between Points on a Square
fn to_1d(side: i64, x: i64, y: i64) -> i64 {
    if x == 0 {
        y
    } else if y == side {
        side + x
    } else if x == side {
        side * 3 - y
    } else {
        side * 4 - x
    }
}

fn check(nums: &[i64], peri: i64, k: i32, lo: i64) -> bool {
    let n = nums.len();
    let mut ext = nums.to_vec();
    ext.extend(nums.iter().map(|&x| x + peri));
    for i in 0..n {
        let start = ext[i];
        let limit = start + peri - lo;
        let mut cur = start;
        let mut ok = true;
        for _ in 1..k {
            let j = ext.partition_point(|&x| x < cur + lo);
            if j == ext.len() || ext[j] > limit {
                ok = false;
                break;
            }
            cur = ext[j];
        }
        if ok {
            return true;
        }
    }
    false
}

fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
    let side = side as i64;
    let mut nums: Vec<i64> = points
        .iter()
        .map(|p| to_1d(side, p[0] as i64, p[1] as i64))
        .collect();
    nums.sort_unstable();
    let peri = side * 4;
    let mut lo = 0i32;
    let mut hi = side as i32;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if check(&nums, peri, k, mid as i64) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn main() {
    println!(
        "{}",
        max_distance(2, vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example1() {
        assert_eq!(
            max_distance(2, vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]], 4),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_distance(
                2,
                vec![vec![0, 0], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]],
                4
            ),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_distance(
                2,
                vec![
                    vec![0, 0],
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 2],
                    vec![2, 0],
                    vec![2, 2],
                    vec![2, 1]
                ],
                5
            ),
            1
        );
    }
}
