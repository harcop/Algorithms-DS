/// LeetCode #2087 - Minimum Cost Homecoming of a Robot in a Grid
fn min_cost(
    start_pos: Vec<i32>,
    home_pos: Vec<i32>,
    row_costs: Vec<i32>,
    col_costs: Vec<i32>,
) -> i32 {
    let mut cost = 0;
    let mut row = start_pos[0];
    let mut col = start_pos[1];

    while row != home_pos[0] {
        row += if row < home_pos[0] { 1 } else { -1 };
        cost += row_costs[row as usize];
    }
    while col != home_pos[1] {
        col += if col < home_pos[1] { 1 } else { -1 };
        cost += col_costs[col as usize];
    }

    cost
}

fn main() {
    println!(
        "{}",
        min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7])
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]),
            18
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_cost(
                vec![0, 0],
                vec![0, 0],
                vec![5, 4, 3],
                vec![8, 2, 6, 7],
            ),
            0
        );
    }

    #[test]
    fn moves_up_and_left() {
        assert_eq!(min_cost(vec![2, 3], vec![0, 1], vec![1, 2, 3], vec![4, 5, 6, 7]), 14);
    }
}
