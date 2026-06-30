/// LeetCode #2182 - Construct String With Repeat Limit
use std::collections::BinaryHeap;

fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }

    let mut heap: BinaryHeap<(u8, i32)> = (0..26)
        .filter(|&i| cnt[i] > 0)
        .map(|i| (b'a' + i as u8, cnt[i]))
        .collect();

    let mut ans = String::new();
    while let Some((ch, mut count)) = heap.pop() {
        let take = count.min(repeat_limit);
        ans.extend(std::iter::repeat(ch as char).take(take as usize));
        count -= take;

        if count > 0 {
            if let Some((ch2, count2)) = heap.pop() {
                ans.push(ch2 as char);
                heap.push((ch, count));
                if count2 > 1 {
                    heap.push((ch2, count2 - 1));
                }
            } else {
                break;
            }
        }
    }

    ans
}

fn main() {
    println!("{}", repeat_limited_string("cczazcc".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::repeat_limited_string;

    #[test]
    fn example_one() {
        assert_eq!(repeat_limited_string("cczazcc".into(), 3), "zzcccac");
    }

    #[test]
    fn example_two() {
        assert_eq!(repeat_limited_string("aababab".into(), 2), "bbabaa");
    }
}
