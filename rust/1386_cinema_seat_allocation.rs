/// LeetCode #1386 - Cinema Seat Allocation
fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut rows: HashMap<i32, i32> = HashMap::new();
    for r in reserved_seats {
        *rows.entry(r[0]).or_insert(0) |= 1 << r[1];
    }
    let mut ans = (n - rows.len() as i32) * 2;
    const LEFT: i32 = 0b111100; // seats 2-5
    const MID: i32 = 0b11110000; // seats 4-7
    const RIGHT: i32 = 0b1111000000; // seats 6-9
    for seats in rows.values_mut() {
        let mut families = 0;
        if *seats & LEFT == 0 {
            families += 1;
            *seats |= LEFT;
        }
        if *seats & MID == 0 {
            families += 1;
            *seats |= MID;
        }
        if *seats & RIGHT == 0 {
            families += 1;
        }
        ans += families;
    }
    ans
}

fn main() {
    println!("{}", max_number_of_families(3, vec![vec![1, 2], vec![1, 3], vec![1, 8], vec![2, 6], vec![3, 1], vec![3, 10]]));
}

#[cfg(test)]
mod tests {
    use super::max_number_of_families;

    #[test]
    fn example_one() {
        assert_eq!(
            max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10],
                ],
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]), 2);
    }
}
