/// LeetCode #3000 - Maximum Area of Longest Diagonal Rectangle
fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut best_diag = 0i64;
    let mut best_area = 0i32;
    for dim in dimensions {
        let l = dim[0] as i64;
        let w = dim[1] as i64;
        let diag = l * l + w * w;
        let area = (l * w) as i32;
        if diag > best_diag || (diag == best_diag && area > best_area) {
            best_diag = diag;
            best_area = area;
        }
    }
    best_area
}

fn main() {
    println!("{}", area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]));
}

#[cfg(test)]
mod tests {
    use super::area_of_max_diagonal;

    #[test]
    fn example_one() {
        assert_eq!(area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]), 48);
    }

    #[test]
    fn example_two() {
        assert_eq!(area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]), 12);
    }
}
