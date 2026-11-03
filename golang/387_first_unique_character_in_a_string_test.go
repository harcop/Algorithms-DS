package leetcode

import "testing"

func TestFirstUniqCharExamples(t *testing.T) {
	if got := firstUniqChar("leetcode"); got != 0 {
		t.Fatalf("firstUniqChar() = %d, want 0", got)
	}
	if got := firstUniqChar("loveleetcode"); got != 2 {
		t.Fatalf("firstUniqChar() = %d, want 2", got)
	}
	if got := firstUniqChar("aabb"); got != -1 {
		t.Fatalf("firstUniqChar() = %d, want -1", got)
	}
}
