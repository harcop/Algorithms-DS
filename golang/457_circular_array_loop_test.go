package leetcode

import "testing"

func TestCircularArrayLoopExamples(t *testing.T) {
	if !circularArrayLoop([]int{2, -1, 1, 2, 2}) {
		t.Fatal("circularArrayLoop([2,-1,1,2,2]) = false, want true")
	}
	if circularArrayLoop([]int{-1, 2}) {
		t.Fatal("circularArrayLoop([-1,2]) = true, want false")
	}
	if circularArrayLoop([]int{-1, -2, -3, -4, -5}) {
		t.Fatal("circularArrayLoop(all negative) = true, want false")
	}
}
