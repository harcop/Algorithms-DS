package leetcode

import "testing"

func TestIsValidExampleOne(t *testing.T) {
	if !isValid("()") {
		t.Fatal(`isValid("()") = false, want true`)
	}
}

func TestIsValidExampleTwo(t *testing.T) {
	if !isValid("()[]{}") {
		t.Fatal(`isValid("()[]{}") = false, want true`)
	}
}

func TestIsValidExampleThree(t *testing.T) {
	if isValid("(]") {
		t.Fatal(`isValid("(]") = true, want false`)
	}
}
