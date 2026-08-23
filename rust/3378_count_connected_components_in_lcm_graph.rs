/// LeetCode #3378 - Count Connected Components in LCM Graph
fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
    let t = threshold as usize;
    let mut parent: Vec<usize> = (0..=t).collect();
    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut [usize], a: usize, b: usize) {
        let pa = find(parent, a);
        let pb = find(parent, b);
        if pa != pb {
            parent[pb] = pa;
        }
    }
    let mut extra = 0;
    for &num in &nums {
        if num > threshold {
            extra += 1;
            continue;
        }
        let num = num as usize;
        let mut j = num;
        while j <= t {
            union(&mut parent, num, j);
            j += num;
        }
    }
    let mut seen = std::collections::HashSet::new();
    for &num in &nums {
        if num <= threshold {
            seen.insert(find(&mut parent, num as usize));
        }
    }
    extra + seen.len() as i32
}

fn main() {
    println!("{}", count_components(vec![2, 4, 8, 3, 9], 5));
}

#[cfg(test)]
mod tests {
    use super::count_components;

    #[test]
    fn example1() {
        assert_eq!(count_components(vec![2, 4, 8, 3, 9], 5), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_components(vec![2, 4, 8, 3, 9, 12], 10), 2);
    }
}
