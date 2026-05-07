/// LeetCode #427 - Construct Quad Tree
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuadTreeNode {
    pub val: bool,
    pub is_leaf: bool,
    pub top_left: Option<Box<QuadTreeNode>>,
    pub top_right: Option<Box<QuadTreeNode>>,
    pub bottom_left: Option<Box<QuadTreeNode>>,
    pub bottom_right: Option<Box<QuadTreeNode>>,
}

fn construct(grid: Vec<Vec<i32>>) -> Option<Box<QuadTreeNode>> {
    fn build(g: &Vec<Vec<i32>>, r0: usize, c0: usize, len: usize) -> Box<QuadTreeNode> {
        let v = g[r0][c0];
        let mut same = true;
        'outer: for i in r0..r0 + len {
            for j in c0..c0 + len {
                if g[i][j] != v {
                    same = false;
                    break 'outer;
                }
            }
        }
        if same {
            return Box::new(QuadTreeNode {
                val: v == 1,
                is_leaf: true,
                top_left: None,
                top_right: None,
                bottom_left: None,
                bottom_right: None,
            });
        }
        let half = len / 2;
        Box::new(QuadTreeNode {
            val: true,
            is_leaf: false,
            top_left: Some(build(g, r0, c0, half)),
            top_right: Some(build(g, r0, c0 + half, half)),
            bottom_left: Some(build(g, r0 + half, c0, half)),
            bottom_right: Some(build(g, r0 + half, c0 + half, half)),
        })
    }
    let n = grid.len();
    if n == 0 { None } else { Some(build(&grid, 0, 0, n)) }
}

fn main() {
    let _ = construct(vec![vec![0, 1], vec![1, 0]]);
}

#[cfg(test)]
mod tests {
    use super::construct;

    #[test]
    fn example_one() {
        let root = construct(vec![vec![0, 1], vec![1, 0]]).unwrap();
        assert!(!root.is_leaf);
        assert!(root.top_left.as_ref().unwrap().is_leaf);
        assert!(!root.top_left.as_ref().unwrap().val);
    }
}
