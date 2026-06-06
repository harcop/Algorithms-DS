/// LeetCode #1776 - Car Fleet II
fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
    let n = cars.len();
    let mut ans = vec![-1.0; n];
    let mut stack: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        let pos = cars[i][0] as f64;
        let spd = cars[i][1] as f64;
        while let Some(j) = stack.last().copied() {
            let pos_j = cars[j][0] as f64;
            let spd_j = cars[j][1] as f64;
            if spd <= spd_j {
                stack.pop();
                continue;
            }
            let t = (pos_j - pos) / (spd - spd_j);
            if ans[j] < 0.0 || t < ans[j] {
                break;
            }
            stack.pop();
        }
        if let Some(&j) = stack.last() {
            let pos_j = cars[j][0] as f64;
            let spd_j = cars[j][1] as f64;
            ans[i] = (pos_j - pos) / (spd - spd_j);
        }
        stack.push(i);
    }
    ans
}
fn main() {
    println!(
        "{:?}",
        get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]])
    );
}
#[cfg(test)]
mod tests {
    use super::get_collision_times;
    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }
    #[test]
    fn example_one() {
        let got = get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]);
        assert!(close(got[0], 1.0));
        assert!(close(got[1], -1.0));
        assert!(close(got[2], 3.0));
        assert!(close(got[3], -1.0));
    }
    #[test]
    fn example_two() {
        let got = get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]]);
        assert!(close(got[0], 2.0));
        assert!(close(got[1], 1.0));
        assert!(close(got[2], 1.5));
        assert!(close(got[3], -1.0));
    }
}
