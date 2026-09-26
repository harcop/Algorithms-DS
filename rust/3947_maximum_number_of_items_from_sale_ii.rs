/// LeetCode #3947 - Maximum Number of Items From Sale II
fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i64 {
    let n = items.len();
    let mut cnt = vec![0i32; n + 1];
    for it in &items {
        cnt[it[0] as usize] += 1;
    }
    let mut mult = vec![0i32; n + 1];
    for f in 1..=n {
        let mut s = 0i32;
        let mut m = f;
        while m <= n {
            s += cnt[m];
            m += f;
        }
        mult[f] = s;
    }
    let mn = items.iter().map(|it| it[1] as i64).min().unwrap();
    let mut order: Vec<(i64, i64)> = items
        .iter()
        .map(|it| (it[1] as i64, mult[it[0] as usize] as i64 - 1))
        .collect();
    order.sort_by_key(|x| x.0);
    let mut b = budget as i64;
    let mut ans = 0i64;
    for (price, gifts) in order {
        if price >= 2 * mn {
            continue;
        }
        let take = (b / price).min(gifts);
        ans += take * 2;
        b -= take * price;
    }
    ans + b / mn
}

fn main() {
    println!(
        "{}",
        maximum_sale_items(vec![vec![1, 6], vec![2, 4], vec![3, 5]], 19)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_sale_items;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_sale_items(vec![vec![1, 6], vec![2, 4], vec![3, 5]], 19),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_sale_items(
                vec![
                    vec![2, 8],
                    vec![1, 10],
                    vec![6, 6],
                    vec![4, 12],
                    vec![5, 20],
                    vec![5, 17]
                ],
                35
            ),
            7
        );
    }
}
