/// LeetCode #4009 - Minimum Possible Maximum Waiting Time
fn min_max_waiting_time(demand: Vec<i32>, fuel: Vec<i32>) -> i32 {
    const INF: i32 = 1_000_000_000;
    let n = demand.len();
    let mut dp = vec![vec![vec![vec![INF; 21]; 21]; 51]; 51];
    dp[fuel[0] as usize][fuel[1] as usize][0][0] = 0;
    let mut served = 0usize;
    let mut best = INF;
    for i in 0..n {
        let d = demand[i] as usize;
        let mut next = vec![vec![vec![vec![INF; 21]; 21]; 51]; 51];
        let mut any = false;
        for a in 0..=50 {
            for b in 0..=50 {
                for x in 0..=20 {
                    for y in 0..=20 {
                        let w = dp[a][b][x][y];
                        if w >= INF {
                            continue;
                        }
                        if a >= d {
                            let wait = x as i32;
                            let nx = d;
                            let ny = if y >= x { y - x } else { 0 };
                            let nw = w.max(wait);
                            if nw < next[a - d][b][nx][ny] {
                                next[a - d][b][nx][ny] = nw;
                                any = true;
                            }
                        }
                        if b >= d {
                            let wait = y as i32;
                            let ny = d;
                            let nx = if x >= y { x - y } else { 0 };
                            let nw = w.max(wait);
                            if nw < next[a][b - d][nx][ny] {
                                next[a][b - d][nx][ny] = nw;
                                any = true;
                            }
                        }
                    }
                }
            }
        }
        if !any {
            break;
        }
        served = i + 1;
        best = INF;
        for a in 0..=50 {
            for b in 0..=50 {
                for x in 0..=20 {
                    for y in 0..=20 {
                        best = best.min(next[a][b][x][y]);
                    }
                }
            }
        }
        dp = next;
    }
    if served == 0 { -1 } else { best }
}

fn main() {
    println!(
        "{}",
        min_max_waiting_time(vec![6, 8, 4, 6, 5], vec![16, 13])
    );
}

#[cfg(test)]
mod tests {
    use super::min_max_waiting_time;

    #[test]
    fn example1() {
        assert_eq!(min_max_waiting_time(vec![6, 8, 4, 6, 5], vec![16, 13]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(min_max_waiting_time(vec![10, 15], vec![12, 17]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_max_waiting_time(vec![10, 5], vec![8, 8]), -1);
    }
}
