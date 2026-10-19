package leetcode

import "testing"

func TestConvertToTitleExampleOne(t *testing.T) {
	got := convertToTitle(1)
	if got != "A" {
		t.Fatalf("convertToTitle() = %q, want %q", got, "A")
	}
}

func TestConvertToTitleExampleTwo(t *testing.T) {
	got := convertToTitle(28)
	if got != "AB" {
		t.Fatalf("convertToTitle() = %q, want %q", got, "AB")
	}
}

func TestConvertToTitleExampleThree(t *testing.T) {
	got := convertToTitle(701)
	if got != "ZY" {
		t.Fatalf("convertToTitle() = %q, want %q", got, "ZY")
	}
}
