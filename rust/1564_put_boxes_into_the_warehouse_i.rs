/// LeetCode #1564 - Put Boxes Into The Warehouse I
fn max_boxes_in_warehouse(mut boxes: Vec<i32>, mut warehouse: Vec<i32>) -> i32 {
    boxes.sort_unstable_by(|a, b| b.cmp(a));
    for i in 1..warehouse.len() {
        warehouse[i] = warehouse[i].min(warehouse[i - 1]);
    }
    let mut l = 0usize;
    let mut r = warehouse.len() - 1;
    let mut ans = 0;
    for b in boxes {
        if l > r {
            break;
        }
        if b <= warehouse[l] {
            ans += 1;
            l += 1;
        } else if b <= warehouse[r] {
            ans += 1;
            r -= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_boxes_in_warehouse(vec![4, 3, 4, 1], vec![5, 3, 3, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_boxes_in_warehouse;

    #[test]
    fn example_one() {
        assert_eq!(max_boxes_in_warehouse(vec![4, 3, 4, 1], vec![5, 3, 3, 4, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_boxes_in_warehouse(vec![1, 2, 2, 3, 4], vec![3, 4, 1, 2]), 3);
    }
}
