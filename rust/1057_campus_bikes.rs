/// LeetCode #1057 - Campus Bikes
fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pairs: Vec<(i32, usize, usize)> = Vec::new();
    for (wi, w) in workers.iter().enumerate() {
        for (bi, b) in bikes.iter().enumerate() {
            let d = (w[0] - b[0]).abs() + (w[1] - b[1]).abs();
            pairs.push((d, wi, bi));
        }
    }
    pairs.sort_unstable();
    let mut ans = vec![-1i32; workers.len()];
    let mut used_bike = vec![false; bikes.len()];
    for (_, wi, bi) in pairs {
        if ans[wi] == -1 && !used_bike[bi] {
            ans[wi] = bi as i32;
            used_bike[bi] = true;
        }
    }
    ans
}

fn main() {
    println!("{:?}", assign_bikes(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::assign_bikes;

    #[test]
    fn example_one() {
        assert_eq!(assign_bikes(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]), vec![1, 0]);
    }
}
