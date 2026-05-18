/// LeetCode #997 - Find the Town Judge
fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut in_deg = vec![0i32; n as usize + 1];
    let mut out_deg = vec![0i32; n as usize + 1];
    for t in trust {
        out_deg[t[0] as usize] += 1;
        in_deg[t[1] as usize] += 1;
    }
    for i in 1..=n as usize {
        if in_deg[i] == n - 1 && out_deg[i] == 0 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", find_judge(2, vec![vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_judge;

    #[test]
    fn example_one() {
        assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }
}
