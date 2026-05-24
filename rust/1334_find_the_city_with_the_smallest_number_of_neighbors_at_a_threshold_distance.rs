/// LeetCode #1334 - Find the City With the Smallest Number of Neighbors at a Threshold Distance
fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let inf = i32::MAX / 4;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        dist[u][v] = e[2];
        dist[v][u] = e[2];
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] < inf && dist[k][j] < inf {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }
    let mut best_city = 0;
    let mut best_cnt = n + 1;
    for i in 0..n {
        let cnt = dist[i].iter().filter(|&&d| d <= distance_threshold).count() - 1;
        if cnt < best_cnt || (cnt == best_cnt && i > best_city) {
            best_cnt = cnt;
            best_city = i;
        }
    }
    best_city as i32
}

fn main() {
    println!(
        "{}",
        find_the_city(4, vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::find_the_city;

    #[test]
    fn example_one() {
        assert_eq!(find_the_city(4, vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]], 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_the_city(5, vec![vec![0, 1, 2], vec![0, 4, 8], vec![1, 2, 3], vec![1, 4, 2], vec![2, 3, 1], vec![3, 4, 3]], 2), 4);
    }
}
