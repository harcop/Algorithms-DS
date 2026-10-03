/// LeetCode #4045 - Count Robot Groups
fn count_robot_groups(position: Vec<i32>, speed: Vec<i32>, distance: i32) -> i32 {
    let n = position.len();
    let mut reps = Vec::new();
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j + 1 < n && position[j + 1] - position[j] <= distance {
            j += 1;
        }
        reps.push(speed[j]);
        i = j + 1;
    }
    let mut leaders = Vec::new();
    for v in reps.into_iter().rev() {
        if leaders.last().is_some_and(|&top| v > top) {
            continue;
        }
        leaders.push(v);
    }
    leaders.len() as i32
}

fn main() {
    println!(
        "{}",
        count_robot_groups(vec![1, 5, 6, 20], vec![4, 3, 2, 3], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::count_robot_groups;

    #[test]
    fn example1() {
        assert_eq!(
            count_robot_groups(vec![1, 5, 6, 20], vec![4, 3, 2, 3], 1),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(count_robot_groups(vec![1, 5, 9], vec![3, 2, 2], 2), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_robot_groups(vec![9], vec![8], 5), 1);
    }
}
