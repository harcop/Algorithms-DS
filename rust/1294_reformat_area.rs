/// LeetCode #1294 - Reformat Area
fn reformat_area(total: i32, width: i32, length: i32) -> Vec<i32> {
    if width > 0 && total % width == 0 && total / width == length {
        vec![length]
    } else {
        vec![]
    }
}

fn main() {
    println!("{:?}", reformat_area(6, 2, 3));
}

#[cfg(test)]
mod tests {
    use super::reformat_area;

    #[test]
    fn example_one() {
        assert_eq!(reformat_area(6, 2, 3), vec![3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(reformat_area(4, 1, 2), vec![]);
    }
}
