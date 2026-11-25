package leetcode

import "testing"

func TestJudgePoint24Examples(t *testing.T) {
	if !judgePoint24([]int{4, 1, 8, 7}) {
		t.Fatal("judgePoint24([4 1 8 7]) = false, want true")
	}
	if judgePoint24([]int{1, 1, 1, 3}) {
		t.Fatal("judgePoint24([1 1 1 3]) = true, want false")
	}
}
