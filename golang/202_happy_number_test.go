package leetcode

import "testing"

func TestIsHappyExampleOne(t *testing.T) {
	if !isHappy(19) {
		t.Fatal("isHappy(19) = false, want true")
	}
}

func TestIsHappyExampleTwo(t *testing.T) {
	if isHappy(2) {
		t.Fatal("isHappy(2) = true, want false")
	}
}
