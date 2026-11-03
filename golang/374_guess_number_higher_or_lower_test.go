package leetcode

import "testing"

func TestGuessNumberExamples(t *testing.T) {
	guessTarget = 6
	if got := guessNumber(10); got != 6 {
		t.Fatalf("guessNumber(10) = %d, want 6", got)
	}
	guessTarget = 1
	if got := guessNumber(1); got != 1 {
		t.Fatalf("guessNumber(1) = %d, want 1", got)
	}
	guessTarget = 1
	if got := guessNumber(2); got != 1 {
		t.Fatalf("guessNumber(2) = %d, want 1", got)
	}
}
