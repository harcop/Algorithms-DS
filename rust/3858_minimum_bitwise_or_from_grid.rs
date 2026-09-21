/// LeetCode #3858 - Minimum Bitwise OR From Grid
fn minimum_or(grid: Vec<Vec<i32>>) -> i32 {
    let mx = grid.iter().flatten().copied().max().unwrap();
    let m = if mx == 0 { 0 } else { 32 - mx.leading_zeros() };
    let mut ans = 0;
    for i in (0..m).rev() {
        let mask = ans | ((1 << i) - 1);
        for row in &grid {
            let found = row.iter().any(|&x| (x | mask) == mask);
            if !found {
                ans |= 1 << i;
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_or(vec![vec![1, 5], vec![2, 4]]));
}

#[cfg(test)]
mod tests {
    use super::minimum_or;

    #[test]
    fn example1() {
        assert_eq!(minimum_or(vec![vec![1, 5], vec![2, 4]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_or(vec![vec![3, 5], vec![6, 4]]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_or(vec![vec![7, 9, 8]]), 7);
    }
}
