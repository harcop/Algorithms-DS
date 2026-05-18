/// LeetCode #1024 - Video Stitching
fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
    let mut max_end = vec![0i32; time as usize + 1];
    for c in clips {
        if c[0] <= time {
            let end = c[1].min(time) as usize;
            max_end[c[0] as usize] = max_end[c[0] as usize].max(end as i32);
        }
    }
    let mut reach = 0i32;
    let mut end = 0i32;
    let mut clips_used = 0i32;
    for i in 0..=time as usize {
        if i as i32 > reach {
            return -1;
        }
        end = end.max(max_end[i]);
        if i as i32 == end {
            clips_used += 1;
            reach = end;
        }
    }
    clips_used
}

fn main() {
    println!(
        "{}",
        video_stitching(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]], 10)
    );
}

#[cfg(test)]
mod tests {
    use super::video_stitching;

    #[test]
    fn example_one() {
        assert_eq!(
            video_stitching(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]], 10),
            3
        );
    }
}
