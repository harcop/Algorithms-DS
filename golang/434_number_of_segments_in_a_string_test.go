package leetcode

import "testing"

func TestCountSegmentsExamples(t *testing.T) {
	if got := countSegments("Hello, my name is John"); got != 5 {
		t.Fatalf("countSegments(sentence) = %d, want 5", got)
	}
	if got := countSegments("   "); got != 0 {
		t.Fatalf("countSegments(spaces) = %d, want 0", got)
	}
	if got := countSegments(""); got != 0 {
		t.Fatalf("countSegments(empty) = %d, want 0", got)
	}
}
