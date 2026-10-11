package leetcode

import "testing"

func TestIsMatchExampleOne(t *testing.T) {
	if isMatch("aa", "a") {
		t.Fatal(`isMatch("aa", "a") = true, want false`)
	}
}

func TestIsMatchExampleTwo(t *testing.T) {
	if !isMatch("aa", "a*") {
		t.Fatal(`isMatch("aa", "a*") = false, want true`)
	}
}

func TestIsMatchExampleThree(t *testing.T) {
	if !isMatch("ab", ".*") {
		t.Fatal(`isMatch("ab", ".*") = false, want true`)
	}
}
