/// LeetCode #3323 - Minimize Connected Groups by Inserting Interval
fn min_connected_groups(mut intervals: Vec<Vec<i32>>, k: i32) -> i32 {
    intervals.sort_by_key(|iv| iv[0]);
    let mut merged: Vec<(i32, i32)> = vec![(intervals[0][0], intervals[0][1])];
    for iv in intervals.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if last.1 < iv[0] {
            merged.push((iv[0], iv[1]));
        } else {
            last.1 = last.1.max(iv[1]);
        }
    }
    let m = merged.len();
    let mut ans = m as i32;
    for (i, &(_, e)) in merged.iter().enumerate() {
        let j = merged.partition_point(|&(st, _)| st < e + k + 1);
        ans = ans.min(m as i32 - (j as i32 - i as i32 - 1));
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_connected_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10]], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::min_connected_groups;

    #[test]
    fn example1() {
        assert_eq!(
            min_connected_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10]], 3),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_connected_groups(vec![vec![5, 10], vec![1, 1], vec![3, 3]], 1),
            3
        );
    }
}
