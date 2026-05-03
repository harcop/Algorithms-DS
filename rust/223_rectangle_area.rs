/// LeetCode #223 - Rectangle Area
fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
    let a = (ax2 - ax1) as i64 * (ay2 - ay1) as i64;
    let b = (bx2 - bx1) as i64 * (by2 - by1) as i64;
    let ix = (ax2.min(bx2) - ax1.max(bx1)) as i64;
    let iy = (ay2.min(by2) - ay1.max(by1)) as i64;
    let overlap = if ix > 0 && iy > 0 { ix * iy } else { 0 };
    (a + b - overlap) as i32
}

fn main() {
    println!("{}", compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
}

#[cfg(test)]
mod tests {
    use super::compute_area;

    #[test]
    fn example_one() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }

    #[test]
    fn example_two() {
        assert_eq!(compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
