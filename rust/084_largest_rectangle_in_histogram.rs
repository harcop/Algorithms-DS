/// LeetCode #84 - Largest Rectangle in Histogram
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
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
    println!("{}", largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::largest_rectangle_area;

    #[test]
    fn example_one() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}
