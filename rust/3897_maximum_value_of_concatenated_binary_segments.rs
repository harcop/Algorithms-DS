/// LeetCode #3897 - Maximum Value of Concatenated Binary Segments

const MOD: i64 = 1_000_000_007;

fn segment_order(a: &(i32, i32), b: &(i32, i32)) -> std::cmp::Ordering {
    let key = |&(x, y): &(i32, i32)| {
        if y == 0 && x > 0 {
            (0, -x, 0)
        } else if x == 0 {
            (2, 0, 0)
        } else {
            (1, -x, y)
        }
    };
    key(a).cmp(&key(b))
}

fn maximum_concatenated_value(nums1: Vec<i32>, nums0: Vec<i32>) -> i32 {
    let n = nums1.len();
    let mut segments: Vec<(i32, i32)> = (0..n).map(|i| (nums1[i], nums0[i])).collect();
    segments.sort_by(segment_order);

    let mut ans = 0i64;
    for &(x, y) in &segments {
        for _ in 0..x {
            ans = (ans * 2 + 1) % MOD;
        }
        for _ in 0..y {
            ans = (ans * 2) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!(
        "{}",
        maximum_concatenated_value(vec![1, 2], vec![1, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_concatenated_value;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_concatenated_value(vec![1, 2], vec![1, 0]),
            14
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_concatenated_value(vec![3, 1], vec![0, 3]),
            120
        );
    }
}
