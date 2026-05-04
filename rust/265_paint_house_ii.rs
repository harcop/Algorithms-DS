/// LeetCode #265 - Paint House II
fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
    if costs.is_empty() {
        return 0;
    }
    let k = costs[0].len();
    let mut prev = costs[0].clone();
    for row in costs.into_iter().skip(1) {
        let mut next = vec![i32::MAX; k];
        for j in 0..k {
            let best = (0..k)
                .filter(|&x| x != j)
                .map(|x| prev[x])
                .min()
                .unwrap();
            next[j] = row[j] + best;
        }
        prev = next;
    }
    *prev.iter().min().unwrap()
}

fn main() {
    println!("{}", min_cost_ii(vec![vec![1, 5, 3], vec![2, 9, 4]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost_ii;

    #[test]
    fn example_one() {
        let c = vec![
            vec![1, 5, 3],
            vec![2, 9, 4],
        ];
        assert_eq!(min_cost_ii(c), 5);
    }
}
