package leetcode

import "testing"

func TestMultiplyExampleOne(t *testing.T) {
	got := multiply("2", "3")
	if got != "6" {
		t.Fatalf("multiply() = %q, want %q", got, "6")
	}
}

func TestMultiplyExampleTwo(t *testing.T) {
	got := multiply("123", "456")
	if got != "56088" {
		t.Fatalf("multiply() = %q, want %q", got, "56088")
	}
}
