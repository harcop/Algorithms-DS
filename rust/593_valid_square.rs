/// LeetCode #593 - Valid Square
fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    fn dist2(a: &[i32], b: &[i32]) -> i32 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        dx * dx + dy * dy
    }
    let pts = [p1, p2, p3, p4];
    let mut d = vec![];
    for i in 0..4 {
        for j in i + 1..4 {
            let dd = dist2(&pts[i], &pts[j]);
            if dd == 0 {
                return false;
            }
            d.push(dd);
        }
    }
    d.sort_unstable();
    d[0] == d[1] && d[0] == d[2] && d[0] == d[3] && d[4] == d[5] && d[4] > d[0]
}

fn main() {
    println!(
        "{}",
        valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::valid_square;

    #[test]
    fn example_one() {
        assert!(valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]));
    }
}
