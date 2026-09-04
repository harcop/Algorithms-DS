/// LeetCode #480 - Sliding Window Median
fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let mut ans = Vec::new();
    for i in 0..=nums.len() - k {
        let mut w = nums[i..i + k].to_vec();
        w.sort_unstable();
        if k % 2 == 1 {
            ans.push(w[k / 2] as f64);
        } else {
            ans.push((w[k / 2 - 1] as f64 + w[k / 2] as f64) / 2.0);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::median_sliding_window;

    fn close(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    #[test]
    fn example_one() {
        let got = median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert!(
            close(&got, &[1.0, -1.0, -1.0, 3.0, 5.0, 6.0]),
            "{:?}",
            got
        );
    }

    #[test]
    fn example_two() {
        let got = median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3);
        assert!(
            close(&got, &[2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0]),
            "{:?}",
            got
        );
    }
}
