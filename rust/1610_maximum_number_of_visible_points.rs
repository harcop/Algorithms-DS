/// LeetCode #1610 - Maximum Number Of Visible Points
fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
    use std::f64::consts::PI;
    let mut ang = vec![];
    let mut same = 0i32;
    for p in points {
        if p[0] == location[0] && p[1] == location[1] {
            same += 1;
            continue;
        }
        let a = ((p[1] - location[1]) as f64).atan2((p[0] - location[0]) as f64);
        ang.push(a);
        ang.push(a + 2.0 * PI);
    }
    if ang.is_empty() {
        return same;
    }
    ang.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let w = angle as f64 * PI / 180.0;
    let mut ans = same as usize;
    let mut j = 0usize;
    for i in 0..ang.len() {
        if j < i {
            j = i;
        }
        while j < ang.len() && ang[j] - ang[i] <= w + 1e-9 {
            j += 1;
        }
        ans = ans.max(j - i + same as usize);
    }
    ans as i32
}

fn main() {
    println!("{}", visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]));
}

#[cfg(test)]
mod tests {
    use super::visible_points;

    #[test]
    fn example_one() {
        assert_eq!(
            visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]),
            3
        );
    }
}
