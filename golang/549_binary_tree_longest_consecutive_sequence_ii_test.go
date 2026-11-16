package leetcode

import "testing"

func TestLongestConsecutiveEitherExamples(t *testing.T) {
	first := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	if got := longestConsecutiveEither(first); got != 2 {
		t.Fatalf("longestConsecutiveEither = %d, want 2", got)
	}
	second := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	if got := longestConsecutiveEither(second); got != 3 {
		t.Fatalf("longestConsecutiveEither = %d, want 3", got)
	}
}
