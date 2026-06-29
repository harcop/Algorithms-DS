/// LeetCode #2158 - Amount of New Area Painted Each Day
fn amount_painted(paint: Vec<Vec<i32>>) -> Vec<i32> {
    let max_end = paint.iter().map(|p| p[1] as usize).max().unwrap_or(0);
    let mut parent: Vec<usize> = (0..=max_end).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    let mut ans = Vec::with_capacity(paint.len());
    for p in paint {
        let mut cur = find(&mut parent, p[0] as usize);
        let end = p[1] as usize;
        let mut painted = 0i32;
        while cur < end {
            painted += 1;
            let next = find(&mut parent, cur + 1);
            parent[cur] = next;
            cur = next;
        }
        ans.push(painted);
    }

    ans
}

fn main() {
    println!(
        "{:?}",
        amount_painted(vec![vec![1, 4], vec![4, 7], vec![5, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::amount_painted;

    #[test]
    fn example_one() {
        assert_eq!(
            amount_painted(vec![vec![1, 4], vec![4, 7], vec![5, 8]]),
            vec![3, 3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            amount_painted(vec![vec![1, 4], vec![5, 8], vec![4, 7]]),
            vec![3, 3, 1]
        );
    }
}
