package leetcode

import "testing"

func TestMinMutationExamples(t *testing.T) {
	if got := minMutation("AACCGGTT", "AACCGGTA", []string{"AACCGGTA"}); got != 1 {
		t.Fatalf("minMutation(one step) = %d, want 1", got)
	}
	bank := []string{"AACCGGTA", "AACCGCTA", "AAACGGTA"}
	if got := minMutation("AACCGGTT", "AAACGGTA", bank); got != 2 {
		t.Fatalf("minMutation(two steps) = %d, want 2", got)
	}
	if got := minMutation("AACCGGTT", "AACCGGTA", nil); got != -1 {
		t.Fatalf("minMutation(missing end) = %d, want -1", got)
	}
	if got := minMutation("AACCGGTT", "AACCGGTT", nil); got != 0 {
		t.Fatalf("minMutation(same gene) = %d, want 0", got)
	}
}
