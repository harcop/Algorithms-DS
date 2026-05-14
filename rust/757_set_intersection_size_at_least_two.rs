/// LeetCode #757 - Set Intersection Size At Least Two
fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|x| x[1]);
    let mut s = -1i32;
    let mut t = -1i32;
    let mut ans = 0i32;
    for v in intervals {
        let lo = v[0];
        let hi = v[1];
        if lo > t {
            ans += 2;
            s = hi - 1;
            t = hi;
        } else if lo > s {
            ans += 1;
            s = t;
            t = hi;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::intersection_size_two;

    #[test]
    fn example_one() {
        assert_eq!(
            intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
            3
        );
    }
}
