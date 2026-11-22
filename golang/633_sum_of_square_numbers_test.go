package leetcode

import "testing"

func TestJudgeSquareSumExamples(t *testing.T) {
	if !judgeSquareSum(5) {
		t.Fatal("judgeSquareSum(5) = false, want true")
	}
	if judgeSquareSum(3) {
		t.Fatal("judgeSquareSum(3) = true, want false")
	}
	if !judgeSquareSum(4) {
		t.Fatal("judgeSquareSum(4) = false, want true")
	}
}
