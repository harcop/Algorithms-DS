/// LeetCode #4027 - Elevator Requests III
fn elevator_requests(_n: i32, start: i32, requests: Vec<Vec<i32>>) -> i64 {
    let m = requests.len();
    let inf = i64::MAX / 4;
    let mut f = vec![vec![0i64; m]; 1 << m];
    for mask in 1..(1 << m) {
        for j in 0..m {
            if mask >> j & 1 == 0 {
                continue;
            }
            f[mask][j] = inf;
            let prev = mask ^ (1 << j);
            if prev == 0 {
                let d = (start - requests[j][1]).abs() as i64;
                f[mask][j] = d.max(requests[j][0] as i64);
            } else {
                for j0 in 0..m {
                    if j0 != j && mask >> j0 & 1 == 1 {
                        let d = (requests[j0][1] - requests[j][1]).abs() as i64;
                        f[mask][j] = f[mask][j].min((f[prev][j0] + d).max(requests[j][0] as i64));
                    }
                }
            }
        }
    }
    let full = (1 << m) - 1;
    (0..m).map(|j| f[full][j]).min().unwrap_or(0)
}

fn main() {
    println!(
        "{}",
        elevator_requests(9, 0, vec![vec![0, 8], vec![6, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::elevator_requests;

    #[test]
    fn example1() {
        assert_eq!(elevator_requests(9, 0, vec![vec![0, 8], vec![6, 5]]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(elevator_requests(8, 5, vec![vec![1, 7], vec![7, 3]]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(
            elevator_requests(7, 3, vec![vec![0, 5], vec![0, 1], vec![6, 3]]),
            8
        );
    }
}
