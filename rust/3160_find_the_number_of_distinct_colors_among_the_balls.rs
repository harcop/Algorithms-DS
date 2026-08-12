/// LeetCode #3160 - Find the Number of Distinct Colors Among the Balls
use std::collections::HashMap;

fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ball_color: HashMap<i32, i32> = HashMap::new();
    let mut color_cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let (x, y) = (q[0], q[1]);
        *color_cnt.entry(y).or_insert(0) += 1;
        if let Some(old) = ball_color.insert(x, y) {
            let e = color_cnt.get_mut(&old).unwrap();
            *e -= 1;
            if *e == 0 {
                color_cnt.remove(&old);
            }
        }
        ans.push(color_cnt.len() as i32);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::query_results;

    #[test]
    fn example1() {
        assert_eq!(
            query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
            vec![1, 2, 2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            query_results(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
            ),
            vec![1, 2, 2, 3, 4]
        );
    }
}
