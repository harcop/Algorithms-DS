/// LeetCode #3061 - Calculate Trapping Rain Water (SQL; Rust analogue)

fn trap_rain_water(heights: Vec<i32>) -> i64 {
    let n = heights.len();
    if n == 0 {
        return 0;
    }

    let mut left_max = vec![0; n];
    left_max[0] = heights[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(heights[i]);
    }

    let mut right_max = vec![0; n];
    right_max[n - 1] = heights[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(heights[i]);
    }

    heights
        .iter()
        .enumerate()
        .map(|(i, &h)| (left_max[i].min(right_max[i]) - h).max(0) as i64)
        .sum()
}

fn main() {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{}", trap_rain_water(heights));
}

#[cfg(test)]
mod tests {
    use super::trap_rain_water;

    #[test]
    fn example() {
        let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap_rain_water(heights), 6);
    }
}
