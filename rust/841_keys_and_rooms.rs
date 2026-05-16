/// LeetCode #841 - Keys and Rooms
fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let n = rooms.len();
    let mut seen = vec![false; n];
    let mut stack = vec![0];
    seen[0] = true;
    let mut count = 1;
    while let Some(u) = stack.pop() {
        for &v in &rooms[u] {
            let v = v as usize;
            if !seen[v] {
                seen[v] = true;
                count += 1;
                stack.push(v);
            }
        }
    }
    count == n
}

fn main() {
    println!("{}", can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
}

#[cfg(test)]
mod tests {
    use super::can_visit_all_rooms;

    #[test]
    fn example_one() {
        assert!(can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
    }

    #[test]
    fn example_two() {
        assert!(!can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]));
    }
}
