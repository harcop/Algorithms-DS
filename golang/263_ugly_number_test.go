package leetcode

import "testing"

func TestIsUglyExampleOne(t *testing.T) {
	if !isUgly(6) {
		t.Fatal("isUgly(6) = false, want true")
	}
}

func TestIsUglyExampleTwo(t *testing.T) {
	if !isUgly(1) {
		t.Fatal("isUgly(1) = false, want true")
	}
}

func TestIsUglyExampleThree(t *testing.T) {
	if isUgly(14) {
		t.Fatal("isUgly(14) = true, want false")
	}
}
