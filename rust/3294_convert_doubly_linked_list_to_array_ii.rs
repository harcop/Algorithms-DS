/// LeetCode #3294 - Convert Doubly Linked List to Array II
struct Node {
    val: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

fn to_array(nodes: &[Node], mut i: usize) -> Vec<i32> {
    while let Some(p) = nodes[i].prev {
        i = p;
    }
    let mut ans = Vec::new();
    loop {
        ans.push(nodes[i].val);
        match nodes[i].next {
            Some(n) => i = n,
            None => break,
        }
    }
    ans
}

fn from_slice(vals: &[i32]) -> Vec<Node> {
    let n = vals.len();
    (0..n)
        .map(|i| Node {
            val: vals[i],
            prev: if i == 0 { None } else { Some(i - 1) },
            next: if i + 1 == n { None } else { Some(i + 1) },
        })
        .collect()
}

fn main() {
    let nodes = from_slice(&[1, 2, 3, 4, 5]);
    println!("{:?}", to_array(&nodes, 4));
}

#[cfg(test)]
mod tests {
    use super::{from_slice, to_array};

    #[test]
    fn example1() {
        let nodes = from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(to_array(&nodes, 4), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example2() {
        let nodes = from_slice(&[4, 5, 6, 7, 8]);
        assert_eq!(to_array(&nodes, 4), vec![4, 5, 6, 7, 8]);
    }

    #[test]
    fn from_middle() {
        let nodes = from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(to_array(&nodes, 2), vec![1, 2, 3, 4, 5]);
    }
}
