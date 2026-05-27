/// LeetCode #1450 - Number Of Students Doing Homework At A Given Time
fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time.iter().zip(end_time.iter()).filter(|(&s, &e)| s <= query_time && query_time <= e).count() as i32
}
fn main() { println!("{}", busy_student(vec![1,2,3], vec![3,2,7], 4)); }
#[cfg(test)]
mod tests {
    use super::busy_student;
    #[test]
    fn example_one() { assert_eq!(busy_student(vec![1,2,3], vec![3,2,7], 4), 1); }
    #[test]
    fn example_two() { assert_eq!(busy_student(vec![4], vec![4], 4), 1); }
}