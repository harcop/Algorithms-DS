/// LeetCode #1311 - Get Watched Videos by Your Friends
use std::collections::VecDeque;

fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32) -> Vec<String> {
    let n = friends.len();
    let mut dist = vec![-1; n];
    let mut q = VecDeque::new();
    dist[id as usize] = 0;
    q.push_back(id as usize);
    while let Some(u) = q.pop_front() {
        for &v in &friends[u] {
            let v = v as usize;
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }
    let maxd = *dist.iter().max().unwrap_or(&0);
    let mut ans = vec![];
    for d in 1..=maxd {
        let mut level = vec![];
        for i in 0..n {
            if dist[i] == d {
                level.extend(watched_videos[i].clone());
            }
        }
        level.sort_unstable();
        ans.extend(level);
    }
    ans
}

fn main() {
    let w = vec![vec!["A".to_string(), "B".to_string()], vec!["C".to_string()], vec!["B".to_string(), "C".to_string()], vec![]];
    let f = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
    println!("{:?}", watched_videos_by_friends(w, f, 1));
}

#[cfg(test)]
mod tests {
    use super::watched_videos_by_friends;

    #[test]
    fn example_one() {
        assert_eq!(
            watched_videos_by_friends(
                vec![vec!["A".to_string(), "B".to_string()], vec!["C".to_string()], vec!["B".to_string(), "C".to_string()], vec![]],
                vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
                1,
            ),
            vec!["A".to_string(), "B".to_string(), "B".to_string(), "C".to_string()],
        );
    }
}
