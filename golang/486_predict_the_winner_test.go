package leetcode

import "testing"

func TestPredictTheWinnerExamples(t *testing.T) {
	if predictTheWinner([]int{1, 5, 2}) {
		t.Fatal("predictTheWinner([1,5,2]) = true, want false")
	}
	if !predictTheWinner([]int{1, 5, 233, 7}) {
		t.Fatal("predictTheWinner([1,5,233,7]) = false, want true")
	}
}
