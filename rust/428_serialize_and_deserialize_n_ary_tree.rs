/// LeetCode #428 - Serialize and Deserialize N-ary Tree (pre-order: val, arity, subtrees...)
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NTreeNode {
    pub val: i32,
    pub children: Vec<NTreeNode>,
}

fn serialize(root: Option<NTreeNode>) -> String {
    fn dfs(n: &NTreeNode, out: &mut Vec<String>) {
        out.push(n.val.to_string());
        out.push(n.children.len().to_string());
        for ch in &n.children {
            dfs(ch, out);
        }
    }
    let mut out = vec![];
    if let Some(n) = root {
        dfs(&n, &mut out);
    }
    out.join(",")
}

fn deserialize(data: String) -> Option<NTreeNode> {
    if data.is_empty() {
        return None;
    }
    let nums: Vec<i32> = data.split(',').filter_map(|t| t.parse().ok()).collect();
    fn build(i: &mut usize, nums: &[i32]) -> NTreeNode {
        let val = nums[*i];
        *i += 1;
        let c = nums[*i] as usize;
        *i += 1;
        let mut ch = Vec::with_capacity(c);
        for _ in 0..c {
            ch.push(build(i, nums));
        }
        NTreeNode {
            val,
            children: ch,
        }
    }
    let mut i = 0usize;
    Some(build(&mut i, &nums))
}

fn main() {
    let root = NTreeNode {
        val: 1,
        children: vec![
            NTreeNode {
                val: 3,
                children: vec![
                    NTreeNode {
                        val: 5,
                        children: vec![],
                    },
                    NTreeNode {
                        val: 6,
                        children: vec![],
                    },
                ],
            },
            NTreeNode {
                val: 2,
                children: vec![],
            },
            NTreeNode {
                val: 4,
                children: vec![],
            },
        ],
    };
    println!("{}", serialize(Some(root.clone())).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rt() {
        let root = NTreeNode {
            val: 1,
            children: vec![
                NTreeNode {
                    val: 3,
                    children: vec![
                        NTreeNode {
                            val: 5,
                            children: vec![],
                        },
                        NTreeNode {
                            val: 6,
                            children: vec![],
                        },
                    ],
                },
                NTreeNode {
                    val: 2,
                    children: vec![],
                },
                NTreeNode {
                    val: 4,
                    children: vec![],
                },
            ],
        };
        let s = serialize(Some(root.clone()));
        assert_eq!(deserialize(s), Some(root));
    }
}
