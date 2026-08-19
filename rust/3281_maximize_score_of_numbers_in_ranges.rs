/// LeetCode #3281 - Maximize Score of Numbers in Ranges
fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
    start.sort_unstable();
    let check = |mi: i64| -> bool {
        let mut last = i64::MIN / 4;
        for &st in &start {
            if last + mi > st as i64 + d as i64 {
                return false;
            }
            last = (st as i64).max(last + mi);
        }
        true
    };
    let mut l = 0i64;
    let mut r = start[start.len() - 1] as i64 + d as i64 - start[0] as i64;
    while l < r {
        let mid = (l + r + 1) >> 1;
        if check(mid) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }
    l as i32
}

fn main() {
    println!("{}", max_possible_score(vec![6, 0, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::max_possible_score;

    #[test]
    fn example1() {
        assert_eq!(max_possible_score(vec![6, 0, 3], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_possible_score(vec![2, 6, 13, 13], 5), 5);
    }
}
