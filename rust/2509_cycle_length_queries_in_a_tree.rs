/// LeetCode #2509 - Cycle Length Queries in a Tree
fn cycle_length_queries(_n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let mut a = q[0];
        let mut b = q[1];
        let mut t = 1;
        while a != b {
            if a > b {
                a >>= 1;
            } else {
                b >>= 1;
            }
            t += 1;
        }
        ans.push(t);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        cycle_length_queries(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::cycle_length_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            cycle_length_queries(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]]),
            vec![4, 5, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(cycle_length_queries(2, vec![vec![1, 2]]), vec![2]);
    }
}
