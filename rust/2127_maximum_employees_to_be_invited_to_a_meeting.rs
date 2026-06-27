/// LeetCode #2127 - Maximum Employees to Be Invited to a Meeting
use std::collections::VecDeque;

fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let favorite: Vec<usize> = favorite.into_iter().map(|x| x as usize).collect();
    let mut indegree = vec![0i32; n];
    for &f in &favorite {
        indegree[f] += 1;
    }

    let mut depth = vec![1i32; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if indegree[i] == 0 {
            q.push_back(i);
        }
    }

    while let Some(u) = q.pop_front() {
        let v = favorite[u];
        depth[v] = depth[v].max(depth[u] + 1);
        indegree[v] -= 1;
        if indegree[v] == 0 {
            q.push_back(v);
        }
    }

    let mut max_cycle = 0;
    let mut pair_sum = 0;
    for i in 0..n {
        if indegree[i] == 0 {
            continue;
        }

        let mut len = 0;
        let mut u = i;
        while indegree[u] > 0 {
            indegree[u] = 0;
            len += 1;
            u = favorite[u];
        }

        if len == 2 {
            pair_sum += depth[i] + depth[favorite[i]];
        } else {
            max_cycle = max_cycle.max(len);
        }
    }

    max_cycle.max(pair_sum)
}

fn main() {
    println!("{}", maximum_invitations(vec![2, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::maximum_invitations;

    #[test]
    fn example_one() {
        assert_eq!(maximum_invitations(vec![2, 2, 1, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_invitations(vec![1, 2, 0]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
    }
}
