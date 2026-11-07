package leetcode

import "testing"

func TestParseTernaryExamples(t *testing.T) {
	if got := parseTernary("T?2:3"); got != "2" {
		t.Fatalf("parseTernary(T?2:3) = %q, want 2", got)
	}
	if got := parseTernary("F?1:T?4:5"); got != "4" {
		t.Fatalf("parseTernary(F?1:T?4:5) = %q, want 4", got)
	}
	if got := parseTernary("T?T?F:5:3"); got != "F" {
		t.Fatalf("parseTernary(T?T?F:5:3) = %q, want F", got)
	}
}
