/// LeetCode #957 - Prison Cells After N Days

fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut cells = cells;
    let mut seen: std::collections::HashMap<Vec<i32>, i32> = std::collections::HashMap::new();
    let mut day = 0i32;
    while day < n {
        if let Some(&start) = seen.get(&cells) {
            let cycle = day - start;
            let rem = (n - day) % cycle;
            for _ in 0..rem {
                cells = next_day(cells);
            }
            return cells;
        }
        seen.insert(cells.clone(), day);
        cells = next_day(cells);
        day += 1;
    }
    cells
}

fn next_day(mut cells: Vec<i32>) -> Vec<i32> {
    let n = cells.len();
    let prev = cells.clone();
    for i in 1..n - 1 {
        cells[i] = if prev[i - 1] == prev[i + 1] { 1 } else { 0 };
    }
    cells[0] = 0;
    cells[n - 1] = 0;
    cells
}

fn main() {
    println!("{:?}", prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7));
}

#[cfg(test)]
mod tests {
    use super::prison_after_n_days;

    #[test]
    fn example_one() {
        assert_eq!(
            prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            vec![0, 0, 1, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            vec![0, 0, 1, 1, 1, 1, 1, 0]
        );
    }
}
