/// LeetCode #765 - Couples Holding Hands
fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
    let n = row.len();
    let mut pos = vec![0i32; n];
    for (i, &v) in row.iter().enumerate() {
        pos[v as usize] = i as i32;
    }
    let mut ans = 0i32;
    for i in (0..n).step_by(2) {
        let x = row[i];
        let y = if x % 2 == 0 { x + 1 } else { x - 1 };
        if row[i + 1] == y {
            continue;
        }
        let j = pos[y as usize] as usize;
        let other = row[i + 1];
        row.swap(i + 1, j);
        pos[other as usize] = j as i32;
        pos[y as usize] = (i + 1) as i32;
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", min_swaps_couples(vec![0, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps_couples;

    #[test]
    fn example_one() {
        assert_eq!(min_swaps_couples(vec![0, 2, 1, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }
}
