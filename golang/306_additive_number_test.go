package leetcode

import "testing"

func TestIsAdditiveNumberExampleOne(t *testing.T) {
	if !isAdditiveNumber("112358") {
		t.Fatal("isAdditiveNumber() = false, want true")
	}
}

func TestIsAdditiveNumberExampleTwo(t *testing.T) {
	if isAdditiveNumber("102") {
		t.Fatal("isAdditiveNumber() = true, want false")
	}
}
