/// LeetCode #4023 - Elevator Requests II
fn min_penalty(_n: i32, start: i32, requests: Vec<i32>) -> i64 {
    let mut floors: Vec<i32> = requests.into_iter().filter(|&f| f != start).collect();
    floors.sort_unstable();
    floors.dedup();
    let left: Vec<i32> = floors.iter().copied().filter(|&f| f < start).rev().collect();
    let right: Vec<i32> = floors.into_iter().filter(|&f| f > start).collect();
    let nl = left.len();
    let nr = right.len();
    if nl == 0 && nr == 0 {
        return 0;
    }
    let inf = i64::MAX / 4;
    let mut time = vec![vec![[inf; 2]; nr + 1]; nl + 1];
    let mut pen = vec![vec![[inf; 2]; nr + 1]; nl + 1];
    if nl > 0 {
        let t = (start - left[0]) as i64;
        time[1][0][0] = t;
        pen[1][0][0] = t;
    }
    if nr > 0 {
        let t = (right[0] - start) as i64;
        time[0][1][1] = t;
        pen[0][1][1] = t;
    }
    for i in 0..=nl {
        for j in 0..=nr {
            for side in 0..2 {
                let t = time[i][j][side];
                if t >= inf {
                    continue;
                }
                let p = pen[i][j][side];
                let cur = if side == 0 { left[i - 1] } else { right[j - 1] };
                if i < nl {
                    let dest = left[i];
                    let nt = t + (cur - dest).abs() as i64;
                    let np = p + nt;
                    let rem = (nl - (i + 1) + nr - j) as i64;
                    let score = np + nt * rem;
                    let old = if time[i + 1][j][0] >= inf {
                        inf
                    } else {
                        pen[i + 1][j][0] + time[i + 1][j][0] * rem
                    };
                    if score < old {
                        time[i + 1][j][0] = nt;
                        pen[i + 1][j][0] = np;
                    }
                }
                if j < nr {
                    let dest = right[j];
                    let nt = t + (cur - dest).abs() as i64;
                    let np = p + nt;
                    let rem = (nl - i + nr - (j + 1)) as i64;
                    let score = np + nt * rem;
                    let old = if time[i][j + 1][1] >= inf {
                        inf
                    } else {
                        pen[i][j + 1][1] + time[i][j + 1][1] * rem
                    };
                    if score < old {
                        time[i][j + 1][1] = nt;
                        pen[i][j + 1][1] = np;
                    }
                }
            }
        }
    }
    let mut ans = inf;
    if nl > 0 {
        ans = ans.min(pen[nl][nr][0]);
    }
    if nr > 0 {
        ans = ans.min(pen[nl][nr][1]);
    }
    ans
}

fn main() {
    println!("{}", min_penalty(6, 4, vec![1, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_penalty;

    #[test]
    fn example1() {
        assert_eq!(min_penalty(6, 4, vec![1, 5]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(min_penalty(8, 3, vec![3, 7, 1]), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(min_penalty(10, 5, vec![0, 2, 9]), 22);
    }
}
