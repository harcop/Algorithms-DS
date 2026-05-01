/// LeetCode #119 - Pascal's Triangle II
fn get_row(row_index: i32) -> Vec<i32> {
    let mut row = vec![1];
    for _ in 0..row_index {
        let mut next = vec![1];
        for w in row.windows(2) {
            next.push(w[0] + w[1]);
        }
        next.push(1);
        row = next;
    }
    row
}

fn main() {
    println!("{:?}", get_row(3));
}

#[cfg(test)]
mod tests {
    use super::get_row;

    #[test]
    fn example_one() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_row(0), vec![1]);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_row(1), vec![1, 1]);
    }
}
