/// LeetCode #1337 - The K Weakest Rows in a Matrix
fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut rows: Vec<(i32, usize)> = mat
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let soldiers = r.iter().take_while(|&&x| x == 1).count();
            (soldiers as i32, i)
        })
        .collect();
    rows.sort_unstable();
    rows.into_iter().take(k as usize).map(|(_, i)| i as i32).collect()
}

fn main() {
    println!("{:?}", k_weakest_rows(vec![vec![1, 1, 0, 0, 0], vec![1, 1, 1, 1, 0], vec![1, 0, 0, 0, 0], vec![1, 1, 0, 0, 0], vec![1, 1, 1, 1, 1]], 3));
}

#[cfg(test)]
mod tests {
    use super::k_weakest_rows;

    #[test]
    fn example_one() {
        assert_eq!(
            k_weakest_rows(vec![vec![1, 1, 0, 0, 0], vec![1, 1, 1, 1, 0], vec![1, 0, 0, 0, 0], vec![1, 1, 0, 0, 0], vec![1, 1, 1, 1, 1]], 3),
            vec![2, 0, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(k_weakest_rows(vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 1, 1], vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]], 2), vec![0, 1]);
    }
}
