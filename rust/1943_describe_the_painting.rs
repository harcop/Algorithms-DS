/// LeetCode #1943 - Describe the Painting
use std::collections::HashMap;

fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut diff: HashMap<i32, i32> = HashMap::new();
    for seg in segments {
        let l = seg[0];
        let r = seg[1];
        let c = seg[2];
        *diff.entry(l).or_insert(0) += c;
        *diff.entry(r).or_insert(0) -= c;
    }
    let mut pts: Vec<(i32, i32)> = diff.into_iter().collect();
    pts.sort_unstable_by_key(|&(k, _)| k);
    let n = pts.len();
    for i in 1..n {
        pts[i].1 += pts[i - 1].1;
    }
    let mut ans = Vec::new();
    for i in 0..n - 1 {
        if pts[i].1 != 0 {
            ans.push(vec![pts[i].0, pts[i + 1].0, pts[i].1]);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        split_painting(vec![vec![1, 4, 5], vec![4, 7, 7], vec![1, 7, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::split_painting;

    #[test]
    fn example_one() {
        assert_eq!(
            split_painting(vec![vec![1, 4, 5], vec![4, 7, 7], vec![1, 7, 9]]),
            vec![vec![1, 4, 14], vec![4, 7, 16]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            split_painting(vec![vec![1, 7, 9], vec![6, 8, 15], vec![8, 10, 7]]),
            vec![
                vec![1, 6, 9],
                vec![6, 7, 24],
                vec![7, 8, 15],
                vec![8, 10, 7]
            ]
        );
    }
}
