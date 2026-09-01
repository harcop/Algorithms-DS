/// LeetCode #3540 - Minimum Time to Visit All Houses
fn min_total_time(forward: Vec<i32>, backward: Vec<i32>, queries: Vec<i32>) -> i64 {
    let n = forward.len();
    let sum_b: i64 = backward.iter().map(|&x| x as i64).sum();
    let mut prefix_f = vec![0i64; n + 1];
    let mut prefix_b = vec![0i64; n];
    for i in 0..n {
        prefix_f[i + 1] = prefix_f[i] + forward[i] as i64;
        prefix_b[i] = if i == 0 { 0 } else { prefix_b[i - 1] } + backward[i] as i64;
    }
    let mut ans = 0i64;
    let mut pos = 0usize;
    for q in queries {
        let q = q as usize;
        let r = if q < pos { prefix_f[n] } else { 0 } + prefix_f[q] - prefix_f[pos];
        let l = if q > pos { sum_b } else { 0 } + prefix_b[pos] - prefix_b[q];
        ans += l.min(r);
        pos = q;
    }
    ans
}

fn main() {
    println!("{}", min_total_time(vec![1, 4, 4], vec![4, 1, 2], vec![1, 2, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_total_time;

    #[test]
    fn example1() {
        assert_eq!(min_total_time(vec![1, 4, 4], vec![4, 1, 2], vec![1, 2, 0, 2]), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_total_time(vec![1, 1, 1, 1], vec![2, 2, 2, 2], vec![1, 2, 3, 0]),
            4
        );
    }
}
