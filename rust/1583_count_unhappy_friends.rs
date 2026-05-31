/// LeetCode #1583 - Count Unhappy Friends
fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut partner = vec![0usize; n];
    for p in pairs {
        partner[p[0] as usize] = p[1] as usize;
        partner[p[1] as usize] = p[0] as usize;
    }
    let mut rank = vec![vec![0usize; n]; n];
    for i in 0..n {
        for (r, &p) in preferences[i].iter().enumerate() {
            rank[i][p as usize] = r;
        }
    }
    let mut ans = 0i32;
    for x in 0..n {
        let y = partner[x];
        for &u in &preferences[x] {
            if u == y as i32 {
                break;
            }
            let v = partner[u as usize];
            if rank[u as usize][x] < rank[u as usize][v] {
                ans += 1;
                break;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        unhappy_friends(
            4,
            vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![2, 1, 0]],
            vec![vec![0, 1], vec![2, 3]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::unhappy_friends;

    #[test]
    fn example_one() {
        assert_eq!(
            unhappy_friends(
                4,
                vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![2, 1, 0]],
                vec![vec![0, 1], vec![2, 3]],
            ),
            0
        );
    }
}
