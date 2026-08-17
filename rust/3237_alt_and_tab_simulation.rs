/// LeetCode #3237 - Alt and Tab Simulation
use std::collections::HashSet;

fn simulation_result(windows: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut s = HashSet::new();
    let mut ans = Vec::new();
    for &q in queries.iter().rev() {
        if s.insert(q) {
            ans.push(q);
        }
    }
    for w in windows {
        if !s.contains(&w) {
            ans.push(w);
        }
    }
    ans
}

fn main() {
    println!("{:?}", simulation_result(vec![1, 2, 3], vec![3, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::simulation_result;

    #[test]
    fn example1() {
        assert_eq!(
            simulation_result(vec![1, 2, 3], vec![3, 3, 2]),
            vec![2, 3, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            simulation_result(vec![1, 4, 2, 3], vec![4, 1, 3]),
            vec![3, 1, 4, 2]
        );
    }
}
