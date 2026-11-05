package leetcode

import "testing"

func TestMinAbbreviationExamples(t *testing.T) {
	if got := minAbbreviation("apple", []string{"blade"}); got != "a4" {
		t.Fatalf("minAbbreviation() = %q, want a4", got)
	}
	if got := minAbbreviation("apple", []string{"plain", "amber", "blade"}); got != "1p3" {
		t.Fatalf("minAbbreviation() = %q, want 1p3", got)
	}
}
