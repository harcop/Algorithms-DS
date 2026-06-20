/// LeetCode #1998 - GCD Sort of an Array
fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        let r = find(parent, parent[x]);
        parent[x] = r;
    }
    parent[x]
}

fn union(parent: &mut [usize], a: usize, b: usize) {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra != rb {
        parent[ra] = rb;
    }
}

fn gcd_sort(nums: Vec<i32>) -> bool {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut factors: Vec<Vec<usize>> = vec![Vec::new(); mx + 1];
    for i in 2..=mx {
        if !factors[i].is_empty() {
            continue;
        }
        for j in (i..=mx).step_by(i) {
            factors[j].push(i);
        }
    }

    let limit = mx + 1;
    let mut parent: Vec<usize> = (0..limit).collect();
    for &num in &nums {
        let num = num as usize;
        for &p in &factors[num] {
            union(&mut parent, num, p);
        }
    }

    let mut sorted = nums.clone();
    sorted.sort_unstable();
    for (i, &num) in nums.iter().enumerate() {
        if sorted[i] != num && find(&mut parent, num as usize) != find(&mut parent, sorted[i] as usize)
        {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", gcd_sort(vec![7, 21, 3]));
}

#[cfg(test)]
mod tests {
    use super::gcd_sort;

    #[test]
    fn example_one() {
        assert!(gcd_sort(vec![7, 21, 3]));
    }

    #[test]
    fn example_two() {
        assert!(!gcd_sort(vec![5, 2, 6, 2]));
    }
}
