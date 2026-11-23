package leetcode

import "testing"

func TestJudgeCircleExamples(t *testing.T) {
	if !judgeCircle("UD") {
		t.Fatal("judgeCircle(UD) = false, want true")
	}
	if judgeCircle("LL") {
		t.Fatal("judgeCircle(LL) = true, want false")
	}
}
