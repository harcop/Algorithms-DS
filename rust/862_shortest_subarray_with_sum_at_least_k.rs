/// LeetCode #862 - Shortest Subarray with Sum at Least K
fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + nums[i] as i64;
    }
    let mut deque = std::collections::VecDeque::new();
    deque.push_back(0);
    let mut ans = n + 1;
    for i in 1..=n {
        while let Some(&j) = deque.front() {
            if prefix[i] - prefix[j] >= k as i64 {
                ans = ans.min(i - j);
                deque.pop_front();
            } else {
                break;
            }
        }
        while let Some(&j) = deque.back() {
            if prefix[j] >= prefix[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);
    }
    if ans == n + 1 {
        -1
    } else {
        ans as i32
    }
}

fn main() {
    println!("{}", shortest_subarray(vec![1], 1));
}

#[cfg(test)]
mod tests {
    use super::shortest_subarray;

    #[test]
    fn example_one() {
        assert_eq!(shortest_subarray(vec![1], 1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_subarray(vec![1, 2, 3], 4), 2);
    }
}
