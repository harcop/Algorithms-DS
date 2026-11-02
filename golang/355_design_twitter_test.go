package leetcode

import "testing"

func TestTwitterExample(t *testing.T) {
	twitter := NewTwitter()
	twitter.PostTweet(1, 5)
	if got := twitter.GetNewsFeed(1); !sameInts(got, []int{5}) {
		t.Fatalf("GetNewsFeed(1) = %v, want [5]", got)
	}
	twitter.Follow(1, 2)
	twitter.PostTweet(2, 6)
	if got := twitter.GetNewsFeed(1); !sameInts(got, []int{6, 5}) {
		t.Fatalf("GetNewsFeed(1) = %v, want [6 5]", got)
	}
	twitter.Unfollow(1, 2)
	if got := twitter.GetNewsFeed(1); !sameInts(got, []int{5}) {
		t.Fatalf("GetNewsFeed(1) = %v, want [5]", got)
	}
}
