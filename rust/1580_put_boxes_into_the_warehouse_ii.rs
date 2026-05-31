/// LeetCode #1580 - Put Boxes Into The Warehouse II
fn max_boxes_in_warehouse(mut boxes: Vec<i32>, mut warehouse: Vec<i32>) -> i32 {
    let n = warehouse.len();
    for i in 1..n {
        warehouse[i] = warehouse[i].min(warehouse[i - 1]);
    }
    for i in (0..n - 1).rev() {
        warehouse[i] = warehouse[i].min(warehouse[i + 1]);
    }
    boxes.sort_unstable_by(|a, b| b.cmp(a));
    let mut ans = 0;
    let mut left = 0usize;
    let mut right = n - 1;
    for b in boxes {
        if left > right {
            break;
        }
        if b <= warehouse[right] {
            ans += 1;
            right -= 1;
        } else if b <= warehouse[left] {
            ans += 1;
            left += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_boxes_in_warehouse(vec![1, 2, 2, 3, 4], vec![3, 4, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_boxes_in_warehouse;

    #[test]
    fn example_one() {
        assert_eq!(max_boxes_in_warehouse(vec![1, 2, 2, 3, 4], vec![3, 4, 1, 2, 3]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_boxes_in_warehouse(vec![3, 5, 5, 2], vec![4, 3, 5, 5, 6]), 2);
    }
}
