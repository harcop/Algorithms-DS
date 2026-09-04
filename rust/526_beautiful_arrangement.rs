/// LeetCode #526 - Beautiful Arrangement
fn count_arrangement(n: i32) -> i32 {
    let n = n as usize;
    let mut used = vec![false; n + 1];
    fn dfs(pos: usize, n: usize, used: &mut [bool]) -> i32 {
        if pos > n {
            return 1;
        }
        let mut cnt = 0;
        for x in 1..=n {
            if !used[x] && (x % pos == 0 || pos % x == 0) {
                used[x] = true;
                cnt += dfs(pos + 1, n, used);
                used[x] = false;
            }
        }
        cnt
    }
    dfs(1, n, &mut used)
}

fn main() {
    println!("{}", count_arrangement(2));
}

#[cfg(test)]
mod tests {
    use super::count_arrangement;

    #[test]
    fn example_one() {
        assert_eq!(count_arrangement(2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_arrangement(1), 1);
    }
}
