/// LeetCode #1503 - Last Moment Before All Ants Fall Out Of A Plank
fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut ans = 0;
    for p in left {
        ans = ans.max(p);
    }
    for p in right {
        ans = ans.max(n - p);
    }
    ans
}

fn main() {
    println!("{}", get_last_moment(4, vec![4, 3], vec![0, 1]));
}

#[cfg(test)]
mod tests {
    use super::get_last_moment;

    #[test]
    fn example_one() {
        assert_eq!(get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_last_moment(7, vec![], vec![0]), 7);
    }
}
