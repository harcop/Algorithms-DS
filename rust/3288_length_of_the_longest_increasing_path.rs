/// LeetCode #3288 - Length of the Longest Increasing Path
fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
    let xk = coordinates[k as usize][0];
    let yk = coordinates[k as usize][1];
    let mut left = Vec::new();
    let mut right = Vec::new();
    for c in &coordinates {
        let x = c[0];
        let y = c[1];
        if x < xk && y < yk {
            left.push((x, y));
        } else if x > xk && y > yk {
            right.push((x, y));
        }
    }
    1 + length_of_lis(&mut left) + length_of_lis(&mut right)
}

fn length_of_lis(coordinates: &mut [(i32, i32)]) -> i32 {
    coordinates.sort_by_key(|&(x, y)| (x, -y));
    let mut tail = Vec::new();
    for &(_, y) in coordinates.iter() {
        let p = tail.partition_point(|&t| t < y);
        if p == tail.len() {
            tail.push(y);
        } else {
            tail[p] = y;
        }
    }
    tail.len() as i32
}

fn main() {
    println!(
        "{}",
        max_path_length(vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::max_path_length;

    #[test]
    fn example1() {
        assert_eq!(
            max_path_length(
                vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]],
                1
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_path_length(vec![vec![2, 1], vec![7, 0], vec![5, 6]], 2),
            2
        );
    }
}
