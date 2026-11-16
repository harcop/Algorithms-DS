package leetcode

import "testing"

func TestFindContestMatchExamples(t *testing.T) {
	if got := findContestMatch(2); got != "(1,2)" {
		t.Fatalf("findContestMatch(2) = %q, want (1,2)", got)
	}
	if got := findContestMatch(4); got != "((1,4),(2,3))" {
		t.Fatalf("findContestMatch(4) = %q, want ((1,4),(2,3))", got)
	}
	if got := findContestMatch(8); got != "(((1,8),(4,5)),((2,7),(3,6)))" {
		t.Fatalf("findContestMatch(8) = %q, want (((1,8),(4,5)),((2,7),(3,6)))", got)
	}
}
