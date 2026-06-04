/// LeetCode #1727 - Largest Submatrix With Rearrangements
fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.is_empty() {
        return 0;
    }
    let mut mat = matrix;
    let cols = mat[0].len();
    for row in &mut mat {
        row.sort_unstable_by(|a, b| b.cmp(a));
    }
    let mut heights = vec![0i32; cols];
    let mut best = 0;
    for row in &mat {
        for j in 0..cols {
            if row[j] == 1 {
                heights[j] += 1;
            } else {
                heights[j] = 0;
            }
        }
        best = best.max(largest_rectangle_area(&heights));
    }
    best
}

fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let n = heights.len();
    for i in 0..=n {
        let cur_h = if i == n { 0 } else { heights[i] };
        while !stack.is_empty() && heights[*stack.last().unwrap()] > cur_h {
            let idx = stack.pop().unwrap();
            let h = heights[idx];
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(h * width);
        }
        if i < n {
            stack.push(i);
        }
    }
    max_area
}
fn main() {
    println!(
        "{}",
        largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]])
    );
}
#[cfg(test)]
mod tests {
    use super::largest_submatrix;
    #[test]
    fn example_one() {
        assert_eq!(
            largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
            4
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(largest_submatrix(vec![vec![1, 0, 1, 0, 1]]), 3);
    }
}
