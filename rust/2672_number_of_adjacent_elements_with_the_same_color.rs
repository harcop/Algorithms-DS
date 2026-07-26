/// LeetCode #2672 - Number of Adjacent Elements With the Same Color
fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut nums = vec![0; n];
    let mut ans = Vec::with_capacity(queries.len());
    let mut x = 0;
    for q in queries {
        let i = q[0] as usize;
        let c = q[1];
        if i > 0 && nums[i] > 0 && nums[i - 1] == nums[i] {
            x -= 1;
        }
        if i + 1 < n && nums[i] > 0 && nums[i + 1] == nums[i] {
            x -= 1;
        }
        if i > 0 && nums[i - 1] == c {
            x += 1;
        }
        if i + 1 < n && nums[i + 1] == c {
            x += 1;
        }
        ans.push(x);
        nums[i] = c;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        color_the_array(4, vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::color_the_array;

    #[test]
    fn example_one() {
        assert_eq!(
            color_the_array(4, vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]]),
            vec![0, 1, 1, 0, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(color_the_array(1, vec![vec![0, 100000]]), vec![0]);
    }
}
