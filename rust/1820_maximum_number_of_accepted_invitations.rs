/// LeetCode #1820 - Maximum Number of Accepted Invitations
use std::collections::HashSet;

fn maximum_invitations(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut match_girl = vec![-1i32; n];

    fn find(
        boy: usize,
        grid: &[Vec<i32>],
        match_girl: &mut [i32],
        vis: &mut HashSet<usize>,
    ) -> bool {
        for girl in 0..grid[0].len() {
            if grid[boy][girl] == 1 && !vis.contains(&girl) {
                vis.insert(girl);
                let matched = match_girl[girl];
                if matched == -1 || find(matched as usize, grid, match_girl, vis) {
                    match_girl[girl] = boy as i32;
                    return true;
                }
            }
        }
        false
    }

    let mut ans = 0i32;
    for boy in 0..m {
        let mut vis = HashSet::new();
        if find(boy, &grid, &mut match_girl, &mut vis) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_invitations(vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![0, 0, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_invitations;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_invitations(vec![
                vec![1, 1, 1],
                vec![1, 0, 1],
                vec![0, 0, 1],
            ]),
            3
        );
    }
}
