/// LeetCode #47 - Permutations II
fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut used = vec![false; nums.len()];
    let mut path = Vec::new();
    let mut out = Vec::new();
    backtrack(&nums, &mut used, &mut path, &mut out);
    out
}

fn backtrack(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        out.push(path.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
            continue;
        }
        used[i] = true;
        path.push(nums[i]);
        backtrack(nums, used, path, out);
        path.pop();
        used[i] = false;
    }
}

fn main() {
    println!("{:?}", permute_unique(vec![1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::permute_unique;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(permute_unique(vec![1, 1, 2]));
        let expected = normalize(vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(permute_unique(vec![1, 2, 3]).len(), 6);
    }
}
