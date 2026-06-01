/// LeetCode #1659 - Maximize Grid Happiness
fn get_max_happy(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let ic_max = introverts_count as usize;
    let ec_max = extroverts_count as usize;
    let mx = 3usize.pow(n as u32);
    let h = [[0, 0, 0], [0, -60, -10], [0, -10, 40]];
    let mut f = vec![0i32; mx];
    let mut g = vec![vec![0i32; mx]; mx];
    let mut bits = vec![vec![0usize; n]; mx];
    let mut ix = vec![0usize; mx];
    let mut ex = vec![0usize; mx];
    for i in 0..mx {
        let mut mask = i;
        for j in 0..n {
            let x = mask % 3;
            mask /= 3;
            bits[i][j] = x;
            match x {
                1 => {
                    ix[i] += 1;
                    f[i] += 120;
                }
                2 => {
                    ex[i] += 1;
                    f[i] += 40;
                }
                _ => {}
            }
            if j > 0 {
                f[i] += h[x][bits[i][j - 1]];
            }
        }
    }
    for i in 0..mx {
        for j in 0..mx {
            for k in 0..n {
                g[i][j] += h[bits[i][k]][bits[j][k]];
            }
        }
    }
    let mut memo = vec![vec![vec![vec![None; ec_max + 1]; ic_max + 1]; mx]; m];
    fn dfs(
        row: usize,
        pre: usize,
        ic: usize,
        ec: usize,
        m: usize,
        mx: usize,
        f: &[i32],
        g: &[Vec<i32>],
        ix: &[usize],
        ex: &[usize],
        memo: &mut [Vec<Vec<Vec<Option<i32>>>>],
    ) -> i32 {
        if row == m || (ic == 0 && ec == 0) {
            return 0;
        }
        if let Some(v) = memo[row][pre][ic][ec] {
            return v;
        }
        let mut ans = 0i32;
        for cur in 0..mx {
            if ix[cur] > ic || ex[cur] > ec {
                continue;
            }
            let val = f[cur] + g[pre][cur]
                + dfs(
                    row + 1,
                    cur,
                    ic - ix[cur],
                    ec - ex[cur],
                    m,
                    mx,
                    f,
                    g,
                    ix,
                    ex,
                    memo,
                );
            ans = ans.max(val);
        }
        memo[row][pre][ic][ec] = Some(ans);
        ans
    }
    dfs(0, 0, ic_max, ec_max, m, mx, &f, &g, &ix, &ex, &mut memo)
}

fn main() {
    println!("{}", get_max_happy(2, 3, 1, 2));
}

#[cfg(test)]
mod tests {
    use super::get_max_happy;

    #[test]
    fn example_one() {
        assert_eq!(get_max_happy(2, 3, 1, 2), 240);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_max_happy(3, 1, 2, 1), 260);
    }
}
