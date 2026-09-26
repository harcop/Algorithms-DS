/// LeetCode #3946 - Maximum Number of Items From Sale I
fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
    let budget = budget as usize;
    let mut f = vec![0i32; budget + 1];
    let mut mn = i32::MAX;
    for item in &items {
        let factor = item[0];
        let price = item[1] as usize;
        mn = mn.min(item[1]);
        let cnt = items.iter().filter(|j| j[0] % factor == 0).count() as i32;
        for j in (price..=budget).rev() {
            f[j] = f[j].max(f[j - price] + cnt);
        }
    }
    (0..=budget)
        .map(|i| f[i] + (budget as i32 - i as i32) / mn)
        .max()
        .unwrap()
}

fn main() {
    println!(
        "{}",
        maximum_sale_items(vec![vec![6, 2], vec![2, 6], vec![3, 4]], 9)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_sale_items;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_sale_items(vec![vec![6, 2], vec![2, 6], vec![3, 4]], 9),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_sale_items(
                vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]],
                8
            ),
            10
        );
    }
}
