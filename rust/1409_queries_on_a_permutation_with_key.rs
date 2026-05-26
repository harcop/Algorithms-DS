/// LeetCode #1409 - Queries On A Permutation With Key
fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut p: Vec<i32> = (1..=m).collect();
    let mut pos = vec![0usize; m as usize + 1];
    for i in 0..m as usize {
        pos[(i + 1) as usize] = i;
    }
    let mut ans = Vec::new();
    for q in queries {
        let idx = pos[q as usize];
        ans.push(idx as i32);
        if idx > 0 {
            let val = p[idx];
            p.remove(idx);
            p.insert(0, val);
            for (i, &v) in p.iter().enumerate() {
                pos[v as usize] = i;
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", process_queries(vec![3, 1, 2, 1], 5));
}

#[cfg(test)]
mod tests {
    use super::process_queries;

    #[test]
    fn example_one() {
        assert_eq!(process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
    }
}

