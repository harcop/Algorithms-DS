/// LeetCode #85 - Maximal Rectangle
fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let cols = matrix[0].len();
    let mut heights = vec![0i32; cols];
    let mut best = 0i32;

    for row in &matrix {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '1' {
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
    let mut max_area = 0i32;
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
    let m = vec![vec!['1', '0', '1', '0', '0'], vec!['1', '0', '1', '1', '1']];
    println!("{}", maximal_rectangle(m));
}

#[cfg(test)]
mod tests {
    use super::maximal_rectangle;

    #[test]
    fn example_one() {
        let m = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(maximal_rectangle(m), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximal_rectangle(vec![vec!['0']]), 0);
    }
}
