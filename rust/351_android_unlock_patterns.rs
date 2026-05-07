/// LeetCode #351 - Android Unlock Patterns (count monotone paths touching k dots, skip rules)
fn number_of_patterns(_m: i32, _n: i32) -> i32 {
    // Full grid-based DFS with skip table (standard LC 351)
    let mut skip = [[0usize; 10]; 10];
    skip[1][3] = 2;
    skip[3][1] = 2;
    skip[1][7] = 4;
    skip[7][1] = 4;
    skip[3][9] = 6;
    skip[9][3] = 6;
    skip[7][9] = 8;
    skip[9][7] = 8;
    skip[1][9] = 5;
    skip[9][1] = 5;
    skip[3][7] = 5;
    skip[7][3] = 5;
    skip[2][8] = 5;
    skip[8][2] = 5;
    skip[4][6] = 5;
    skip[6][4] = 5;

    fn dfs(curr: usize, remain: usize, visited: usize, skip: &[[usize; 10]; 10]) -> i32 {
        if remain == 0 {
            return 1;
        }
        let mut sum = 0;
        for nxt in 1..=9 {
            let bit = 1usize << nxt;
            if visited & bit != 0 {
                continue;
            }
            let cand = visited | bit;
            let sk = skip[curr][nxt];
            if sk == 0 || cand & (1usize << sk) != 0 {
                sum += dfs(nxt, remain - 1, cand, skip);
            }
        }
        sum
    }

    fn count_from(start: usize, len: usize, skip: &[[usize; 10]; 10]) -> i32 {
        dfs(start, len - 1, 1usize << start, skip)
    }

    let m = _m as usize;
    let n = _n as usize;
    let mut total = 0i32;
    for len in m..=n {
        total += count_from(1, len, &skip) * 4
            + count_from(2, len, &skip) * 4
            + count_from(5, len, &skip);
    }
    total
}

fn main() {
    println!("{}", number_of_patterns(1, 1));
}

#[cfg(test)]
mod tests {
    use super::number_of_patterns;

    #[test]
    fn known_small() {
        assert_eq!(number_of_patterns(1, 2), 65);
        assert_eq!(number_of_patterns(3, 3), 320);
    }
}
