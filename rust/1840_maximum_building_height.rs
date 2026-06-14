/// LeetCode #1840 - Maximum Building Height
fn max_building(n: i32, mut restrictions: Vec<Vec<i32>>) -> i32 {
    restrictions.push(vec![1, 0]);
    restrictions.sort_by_key(|r| r[0]);
    if restrictions.last().unwrap()[0] != n {
        restrictions.push(vec![n, n - 1]);
    }
    let m = restrictions.len();
    for i in 1..m {
        restrictions[i][1] = restrictions[i][1]
            .min(restrictions[i - 1][1] + restrictions[i][0] - restrictions[i - 1][0]);
    }
    for i in (1..m - 1).rev() {
        restrictions[i][1] = restrictions[i][1]
            .min(restrictions[i + 1][1] + restrictions[i + 1][0] - restrictions[i][0]);
    }
    let mut ans = 0i32;
    for i in 0..m - 1 {
        let t = (restrictions[i][1]
            + restrictions[i + 1][1]
            + restrictions[i + 1][0]
            - restrictions[i][0])
            / 2;
        ans = ans.max(t);
    }
    ans
}

fn main() {
    println!("{}", max_building(5, vec![vec![2, 1], vec![4, 1]]));
}

#[cfg(test)]
mod tests {
    use super::max_building;

    #[test]
    fn example_one() {
        assert_eq!(max_building(5, vec![vec![2, 1], vec![4, 1]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_building(6, vec![vec![5, 0], vec![2, 1]]), 2);
    }
}
