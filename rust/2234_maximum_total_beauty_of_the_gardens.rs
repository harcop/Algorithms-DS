/// LeetCode #2234 - Maximum Total Beauty of the Gardens
fn maximum_beauty(
    mut flowers: Vec<i32>,
    mut new_flowers: i64,
    target: i32,
    full: i32,
    partial: i32,
) -> i64 {
    flowers.sort_unstable();
    let n = flowers.len();
    let mut prefix = vec![0i64; n + 1];
    for (i, &x) in flowers.iter().enumerate() {
        prefix[i + 1] = prefix[i] + x as i64;
    }

    let mut complete = n;
    while complete > 0 && flowers[complete - 1] >= target {
        complete -= 1;
    }
    let start = n - complete;

    let mut ans = 0i64;
    for x in start..=n {
        if x > start {
            new_flowers -= (target - flowers[n - x]).max(0) as i64;
        }
        if new_flowers < 0 {
            break;
        }

        let mut l = 0i32;
        let mut r = if x >= n {
            -1
        } else {
            (n - x - 1) as i32
        };
        while l < r {
            let mid = (l + r + 1) / 2;
            let cost = flowers[mid as usize] as i64 * (mid as i64 + 1) - prefix[mid as usize + 1];
            if cost <= new_flowers {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        let mut y = 0i64;
        if r >= 0 {
            let cost = flowers[l as usize] as i64 * (l as i64 + 1) - prefix[l as usize + 1];
            y = (flowers[l as usize] as i64 + (new_flowers - cost) / (l as i64 + 1))
                .min(target as i64 - 1);
        }
        ans = ans.max(x as i64 * full as i64 + y * partial as i64);
    }

    ans
}

fn main() {
    println!(
        "{}",
        maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_beauty;

    #[test]
    fn example_one() {
        assert_eq!(maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_beauty(vec![2, 4, 5, 3], 10, 5, 2, 6), 30);
    }
}
