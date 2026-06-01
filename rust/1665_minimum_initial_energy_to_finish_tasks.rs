/// LeetCode #1665 - Minimum Initial Energy To Finish Tasks
fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let mut t = tasks;
    t.sort_unstable_by_key(|v| v[1]);
    let mut energy = 0i32;
    for v in t {
        if energy + v[0] < v[1] { energy = v[1]; } else { energy += v[0]; }
    }
    energy
}
fn main() { println!("{}", minimum_effort(vec![vec![1,2],vec![2,4],vec![4,8]])); }
#[cfg(test)]
mod tests {
    use super::minimum_effort;
    #[test]
    fn example_one() { assert_eq!(minimum_effort(vec![vec![1,2],vec![2,4],vec![4,8]]), 8); }
}