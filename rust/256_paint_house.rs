/// LeetCode #256 - Paint House
fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for row in costs {
        let na = row[0] + b.min(c);
        let nb = row[1] + a.min(c);
        let nc = row[2] + a.min(b);
        a = na;
        b = nb;
        c = nc;
    }
    a.min(b).min(c)
}

fn main() {
    println!("{}", min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
            10
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![vec![7, 6, 2]]), 2);
    }
}
