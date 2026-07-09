/// LeetCode #2326 - Spiral Matrix IV
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let m = m as usize;
    let n = n as usize;
    let mut ans = vec![vec![-1; n]; m];
    let dirs = [0i32, 1, 0, -1, 0];
    let mut i = 0usize;
    let mut j = 0usize;
    let mut k = 0usize;

    loop {
        let Some(node) = head.as_mut() else {
            break;
        };
        ans[i][j] = node.val;
        head = node.next.take();
        if head.is_none() {
            break;
        }
        loop {
            let x = i as i32 + dirs[k];
            let y = j as i32 + dirs[k + 1];
            if x >= 0
                && x < m as i32
                && y >= 0
                && y < n as i32
                && ans[x as usize][y as usize] == -1
            {
                i = x as usize;
                j = y as usize;
                break;
            }
            k = (k + 1) % 4;
        }
    }
    ans
}

fn list_from_vec(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &v in values.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn main() {
    let head = list_from_vec(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
    println!("{:?}", spiral_matrix(3, 5, head));
}

#[cfg(test)]
mod tests {
    use super::{list_from_vec, spiral_matrix};

    #[test]
    fn example_one() {
        let head = list_from_vec(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
        assert_eq!(
            spiral_matrix(3, 5, head),
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7]
            ]
        );
    }

    #[test]
    fn example_two() {
        let head = list_from_vec(&[0, 1, 2]);
        assert_eq!(spiral_matrix(1, 4, head), vec![vec![0, 1, 2, -1]]);
    }
}
