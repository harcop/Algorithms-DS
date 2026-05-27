/// LeetCode #1476 - Subrectangle Queries
pub struct SubrectangleQueries {
    grid: Vec<Vec<i32>>,
}
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { grid: rectangle }
    }
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for r in row1..=row2 {
            for c in col1..=col2 {
                self.grid[r as usize][c as usize] = new_value;
            }
        }
    }
    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.grid[row as usize][col as usize]
    }
}
fn main() {
    let mut s = SubrectangleQueries::new(vec![vec![1,2,1],vec![4,3,2],vec![1,1,1],vec![2,2,2]]);
    s.update_subrectangle(0, 0, 3, 2, 5);
    println!("{}", s.get_value(0, 2));
}
#[cfg(test)]
mod tests {
    use super::SubrectangleQueries;
    #[test]
    fn example_one() {
        let mut s = SubrectangleQueries::new(vec![vec![1,2,1],vec![4,3,2],vec![1,1,1],vec![2,2,2],vec![3,1,3],vec![1,2,2]]);
        s.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(s.get_value(0, 2), 5);
        assert_eq!(s.get_value(3, 1), 5);
        s.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(s.get_value(3, 1), 10);
        assert_eq!(s.get_value(0, 2), 5);
    }
}