package leetcode

import "sort"

// LeetCode #355 - Design Twitter
//
// Each user keeps their own tweets; the feed is the ten newest posts from them and who they follow.
type Twitter struct {
	time    int
	tweets  map[int][][2]int
	follows map[int]map[int]struct{}
}

func NewTwitter() *Twitter {
	return &Twitter{
		tweets:  map[int][][2]int{},
		follows: map[int]map[int]struct{}{},
	}
}

func (t *Twitter) PostTweet(userID, tweetID int) {
	t.time++
	t.tweets[userID] = append(t.tweets[userID], [2]int{t.time, tweetID})
}

func (t *Twitter) GetNewsFeed(userID int) []int {
	users := map[int]struct{}{userID: {}}
	for followee := range t.follows[userID] {
		users[followee] = struct{}{}
	}
	var posts [][2]int
	for user := range users {
		tweets := t.tweets[user]
		start := len(tweets) - 10
		if start < 0 {
			start = 0
		}
		posts = append(posts, tweets[start:]...)
	}
	sort.Slice(posts, func(i, j int) bool { return posts[i][0] > posts[j][0] })
	if len(posts) > 10 {
		posts = posts[:10]
	}
	out := make([]int, len(posts))
	for i, post := range posts {
		out[i] = post[1]
	}
	return out
}

func (t *Twitter) Follow(followerID, followeeID int) {
	if followerID == followeeID {
		return
	}
	if t.follows[followerID] == nil {
		t.follows[followerID] = map[int]struct{}{}
	}
	t.follows[followerID][followeeID] = struct{}{}
}

func (t *Twitter) Unfollow(followerID, followeeID int) {
	delete(t.follows[followerID], followeeID)
}
