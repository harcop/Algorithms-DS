/// LeetCode #3279 - Maximum Total Area Occupied by Pistons
use std::collections::BTreeMap;

fn max_area(height: i32, positions: Vec<i32>, directions: String) -> i64 {
    let mut delta: BTreeMap<i32, i64> = BTreeMap::new();
    let mut diff = 0i64;
    let mut res = 0i64;
    for (pos, dir) in positions.into_iter().zip(directions.bytes()) {
        res += pos as i64;
        if dir == b'U' {
            diff += 1;
            *delta.entry(height - pos).or_insert(0) -= 2;
            *delta.entry(height * 2 - pos).or_insert(0) += 2;
        } else {
            diff -= 1;
            *delta.entry(pos).or_insert(0) += 2;
            *delta.entry(height + pos).or_insert(0) -= 2;
        }
    }
    let mut ans = res;
    let mut pre = 0i32;
    for (&cur, &d) in &delta {
        res += (cur - pre) as i64 * diff;
        pre = cur;
        diff += d;
        ans = ans.max(res);
    }
    ans
}

fn main() {
    println!("{}", max_area(5, vec![2, 5], "UD".into()));
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn example1() {
        assert_eq!(max_area(5, vec![2, 5], "UD".into()), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_area(6, vec![0, 0, 6, 3], "UUDU".into()), 15);
    }
}
