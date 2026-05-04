/// LeetCode #252 - Meeting Rooms
fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    let mut v = intervals;
    v.sort_by_key(|x| x[0]);
    for i in 1..v.len() {
        if v[i][0] < v[i - 1][1] {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
}

#[cfg(test)]
mod tests {
    use super::can_attend_meetings;

    #[test]
    fn example_one() {
        assert!(!can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]));
    }

    #[test]
    fn example_two() {
        assert!(can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
    }
}
