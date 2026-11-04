package leetcode

import "testing"

func TestValidUtf8Examples(t *testing.T) {
	if !validUtf8([]int{197, 130, 1}) {
		t.Fatal("validUtf8() = false, want true")
	}
	if validUtf8([]int{235, 140, 4}) {
		t.Fatal("validUtf8() = true, want false")
	}
}
