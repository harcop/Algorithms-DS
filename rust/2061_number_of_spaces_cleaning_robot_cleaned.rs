/// LeetCode #2061 - Number of Spaces Cleaning Robot Cleaned
use std::collections::HashSet;

fn number_of_clean_rooms(mut room: Vec<Vec<i32>>) -> i32 {
    let rows = room.len();
    let cols = room[0].len();
    let dirs = [0i32, 1, 0, -1, 0];
    let mut vis = HashSet::new();
    let mut ans = 0i32;

    fn dfs(
        i: usize,
        j: usize,
        k: usize,
        room: &mut [Vec<i32>],
        rows: usize,
        cols: usize,
        dirs: &[i32; 5],
        vis: &mut HashSet<(usize, usize, usize)>,
        ans: &mut i32,
    ) {
        if vis.contains(&(i, j, k)) {
            return;
        }
        *ans += (room[i][j] == 0) as i32;
        room[i][j] = -1;
        vis.insert((i, j, k));
        let ni = i as i32 + dirs[k];
        let nj = j as i32 + dirs[k + 1];
        if ni >= 0
            && nj >= 0
            && (ni as usize) < rows
            && (nj as usize) < cols
            && room[ni as usize][nj as usize] != 1
        {
            dfs(ni as usize, nj as usize, k, room, rows, cols, dirs, vis, ans);
        } else {
            dfs(i, j, (k + 1) % 4, room, rows, cols, dirs, vis, ans);
        }
    }

    dfs(0, 0, 0, &mut room, rows, cols, &dirs, &mut vis, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        number_of_clean_rooms(vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_clean_rooms;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_clean_rooms(vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]]),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_of_clean_rooms(vec![vec![0, 1, 0], vec![1, 0, 0], vec![0, 0, 0]]),
            1
        );
    }
}
