package leetcode

import "testing"

func TestMinStickersExamples(t *testing.T) {
	got := minStickers([]string{"with", "example", "science"}, "thehat")
	if got != 3 {
		t.Fatalf("minStickers(thehat) = %d, want 3", got)
	}
	got = minStickers([]string{"notice", "possible"}, "basicbasic")
	if got != -1 {
		t.Fatalf("minStickers(basicbasic) = %d, want -1", got)
	}
}
