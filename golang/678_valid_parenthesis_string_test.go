package leetcode

import "testing"

func TestCheckValidStringExamples(t *testing.T) {
	if !checkValidString("()") {
		t.Fatal(`checkValidString("()") = false, want true`)
	}
	if !checkValidString("(*)") {
		t.Fatal(`checkValidString("(*)") = false, want true`)
	}
	if !checkValidString("(*))") {
		t.Fatal(`checkValidString("(*))") = false, want true`)
	}
}
