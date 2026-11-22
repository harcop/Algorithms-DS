package leetcode

import "testing"

func TestNumDecodingsIIExamples(t *testing.T) {
	if got := numDecodingsII("*"); got != 9 {
		t.Fatalf("numDecodingsII(*) = %d, want 9", got)
	}
	if got := numDecodingsII("1*"); got != 18 {
		t.Fatalf("numDecodingsII(1*) = %d, want 18", got)
	}
	if got := numDecodingsII("2*"); got != 15 {
		t.Fatalf("numDecodingsII(2*) = %d, want 15", got)
	}
	if got := numDecodingsII("**"); got != 96 {
		t.Fatalf("numDecodingsII(**) = %d, want 96", got)
	}
}
