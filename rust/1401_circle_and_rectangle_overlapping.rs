/// LeetCode #1401 - Circle And Rectangle Overlapping
fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let cx = x_center.clamp(x1, x2);
    let cy = y_center.clamp(y1, y2);
    let dx = (x_center - cx) as i64;
    let dy = (y_center - cy) as i64;
    let r = radius as i64;
    dx * dx + dy * dy <= r * r
}

fn main() {
    println!("{}", check_overlap(1, 0, 0, 1, -1, 3, 1));
}

#[cfg(test)]
mod tests {
    use super::check_overlap;

    #[test]
    fn example_one() {
        assert!(check_overlap(1, 0, 0, 1, -1, 3, 1));
    }

    #[test]
    fn example_two() {
        assert!(check_overlap(1, 0, 0, -1, 0, 0, 0));
    }
}

