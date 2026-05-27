/// LeetCode #1499 - Max Value Of Equation
fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut pts = points;
    pts.sort_by_key(|p| p[0]);
    let mut best = i32::MIN;
    for i in 0..pts.len() {
        for j in 0..i {
            if pts[i][0] - pts[j][0] > k { continue; }
            best = best.max(pts[i][1] + pts[j][1] + pts[i][0] - pts[j][0]);
        }
    }
    best
}
fn main() { println!("{}", find_max_value_of_equation(vec![vec![1,3],vec![2,0],vec![5,10],vec![6,-10]], 2)); }
#[cfg(test)]
mod tests {
    use super::find_max_value_of_equation;
    #[test]
    fn example_one() { assert_eq!(find_max_value_of_equation(vec![vec![1,3],vec![2,0],vec![5,10],vec![6,-10]], 2), 4); }
}