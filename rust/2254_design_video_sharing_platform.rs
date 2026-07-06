/// LeetCode #2254 - Design Video Sharing Platform
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct VideoSharingPlatform {
    curr_video_id: i32,
    used_ids: BinaryHeap<Reverse<i32>>,
    video_id_to_video: HashMap<i32, String>,
    video_id_to_views: HashMap<i32, i32>,
    video_id_to_likes: HashMap<i32, i32>,
    video_id_to_dislikes: HashMap<i32, i32>,
}

impl VideoSharingPlatform {
    fn new() -> Self {
        VideoSharingPlatform {
            curr_video_id: 0,
            used_ids: BinaryHeap::new(),
            video_id_to_video: HashMap::new(),
            video_id_to_views: HashMap::new(),
            video_id_to_likes: HashMap::new(),
            video_id_to_dislikes: HashMap::new(),
        }
    }

    fn upload(&mut self, video: String) -> i32 {
        let video_id = self.get_video_id();
        self.video_id_to_video.insert(video_id, video);
        video_id
    }

    fn remove(&mut self, video_id: i32) {
        if self.video_id_to_video.remove(&video_id).is_some() {
            self.used_ids.push(Reverse(video_id));
            self.video_id_to_views.remove(&video_id);
            self.video_id_to_likes.remove(&video_id);
            self.video_id_to_dislikes.remove(&video_id);
        }
    }

    fn watch(&mut self, video_id: i32, start_minute: i32, end_minute: i32) -> String {
        let Some(video) = self.video_id_to_video.get(&video_id) else {
            return "-1".to_string();
        };
        *self.video_id_to_views.entry(video_id).or_insert(0) += 1;
        let start = start_minute as usize;
        let end = (end_minute + 1).min(video.len() as i32) as usize;
        video[start..end].to_string()
    }

    fn like(&mut self, video_id: i32) {
        if self.video_id_to_video.contains_key(&video_id) {
            *self.video_id_to_likes.entry(video_id).or_insert(0) += 1;
        }
    }

    fn dislike(&mut self, video_id: i32) {
        if self.video_id_to_video.contains_key(&video_id) {
            *self.video_id_to_dislikes.entry(video_id).or_insert(0) += 1;
        }
    }

    fn get_likes_and_dislikes(&self, video_id: i32) -> Vec<i32> {
        if self.video_id_to_video.contains_key(&video_id) {
            vec![
                *self.video_id_to_likes.get(&video_id).unwrap_or(&0),
                *self.video_id_to_dislikes.get(&video_id).unwrap_or(&0),
            ]
        } else {
            vec![-1]
        }
    }

    fn get_views(&self, video_id: i32) -> i32 {
        if self.video_id_to_video.contains_key(&video_id) {
            *self.video_id_to_views.get(&video_id).unwrap_or(&0)
        } else {
            -1
        }
    }

    fn get_video_id(&mut self) -> i32 {
        if let Some(Reverse(id)) = self.used_ids.pop() {
            return id;
        }
        let id = self.curr_video_id;
        self.curr_video_id += 1;
        id
    }
}

fn main() {
    let mut platform = VideoSharingPlatform::new();
    println!("{}", platform.upload("123".to_string()));
}

#[cfg(test)]
mod tests {
    use super::VideoSharingPlatform;

    #[test]
    fn example_one() {
        let mut platform = VideoSharingPlatform::new();
        assert_eq!(platform.upload("123".to_string()), 0);
        assert_eq!(platform.upload("456".to_string()), 1);
        platform.remove(4);
        platform.remove(0);
        assert_eq!(platform.upload("789".to_string()), 0);
        assert_eq!(platform.watch(1, 0, 5), "456");
        assert_eq!(platform.watch(1, 0, 1), "45");
        platform.like(1);
        platform.dislike(1);
        platform.dislike(1);
        assert_eq!(platform.get_likes_and_dislikes(1), vec![1, 2]);
        assert_eq!(platform.get_views(1), 2);
    }

    #[test]
    fn example_two() {
        let mut platform = VideoSharingPlatform::new();
        platform.remove(0);
        assert_eq!(platform.watch(0, 0, 1), "-1");
        platform.like(0);
        platform.dislike(0);
        assert_eq!(platform.get_likes_and_dislikes(0), vec![-1]);
        assert_eq!(platform.get_views(0), -1);
    }
}
