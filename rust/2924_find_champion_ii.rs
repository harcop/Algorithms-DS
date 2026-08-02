/// LeetCode #2924 - Find Champion II
fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut indeg = vec![0; n];
    for e in edges {
        indeg[e[1] as usize] += 1;
    }
    let zeros: Vec<_> = indeg
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i as i32)
        .collect();
    if zeros.len() == 1 {
        zeros[0]
    } else {
        -1
    }
}

fn main() {
    println!("{}", find_champion(3, vec![vec![0, 1], vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_champion;

    #[test]
    fn example_one() {
        assert_eq!(find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]),
            -1
        );
    }
}
