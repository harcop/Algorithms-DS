/// LeetCode #2345 - Finding the Number of Visible Mountains
use std::collections::HashMap;

fn visible_mountains(peaks: Vec<Vec<i32>>) -> i32 {
    let mut arr: Vec<(i32, i32)> = peaks
        .iter()
        .map(|p| (p[0] - p[1], p[0] + p[1]))
        .collect();
    let mut cnt: HashMap<(i32, i32), i32> = HashMap::new();
    for &p in &arr {
        *cnt.entry(p).or_insert(0) += 1;
    }
    arr.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut ans = 0;
    let mut cur = i32::MIN;
    for &(l, r) in &arr {
        if r <= cur {
            continue;
        }
        cur = r;
        if cnt[&(l, r)] == 1 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        visible_mountains(vec![vec![2, 2], vec![6, 3], vec![5, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::visible_mountains;

    #[test]
    fn example_one() {
        assert_eq!(
            visible_mountains(vec![vec![2, 2], vec![6, 3], vec![5, 4]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(visible_mountains(vec![vec![1, 3], vec![1, 3]]), 0);
    }
}
