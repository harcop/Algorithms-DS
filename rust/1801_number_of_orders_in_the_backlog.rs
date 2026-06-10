/// LeetCode #1801 - Number of Orders in the Backlog
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i64 = 1_000_000_007;

fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut sell: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    for o in orders {
        let p = o[0];
        let mut a = o[1];
        let t = o[2];
        if t == 0 {
            while a > 0 {
                if let Some(Reverse((sp, _))) = sell.peek().copied() {
                    if sp <= p {
                        let Reverse((sp, sa)) = sell.pop().unwrap();
                        if a >= sa {
                            a -= sa;
                        } else {
                            sell.push(Reverse((sp, sa - a)));
                            a = 0;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if a > 0 {
                buy.push((p, a));
            }
        } else {
            while a > 0 {
                if let Some(&(bp, _)) = buy.peek() {
                    if bp >= p {
                        let (bp, ba) = buy.pop().unwrap();
                        if a >= ba {
                            a -= ba;
                        } else {
                            buy.push((bp, ba - a));
                            a = 0;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if a > 0 {
                sell.push(Reverse((p, a)));
            }
        }
    }

    let total: i64 = buy.iter().map(|&(_, v)| v as i64).sum::<i64>()
        + sell.iter().map(|r| r.0.1 as i64).sum::<i64>();
    (total % MOD) as i32
}

fn main() {
    println!(
        "{}",
        get_number_of_backlog_orders(vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::get_number_of_backlog_orders;

    #[test]
    fn example_one() {
        assert_eq!(
            get_number_of_backlog_orders(vec![
                vec![10, 5, 0],
                vec![15, 2, 1],
                vec![25, 1, 1],
                vec![30, 4, 0],
            ]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_number_of_backlog_orders(vec![
                vec![7, 1_000_000_000, 1],
                vec![15, 3, 0],
                vec![5, 999_999_995, 0],
                vec![5, 1, 1],
            ]),
            999_999_984
        );
    }
}
