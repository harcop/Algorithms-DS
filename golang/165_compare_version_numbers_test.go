package leetcode

import "testing"

func TestCompareVersionExampleOne(t *testing.T) {
	got := compareVersion("1.01", "1.001")
	if got != 0 {
		t.Fatalf("compareVersion() = %d, want 0", got)
	}
}

func TestCompareVersionExampleTwo(t *testing.T) {
	got := compareVersion("1.0", "1.0.0")
	if got != 0 {
		t.Fatalf("compareVersion() = %d, want 0", got)
	}
}

func TestCompareVersionExampleThree(t *testing.T) {
	got := compareVersion("0.1", "1.1")
	if got != -1 {
		t.Fatalf("compareVersion() = %d, want -1", got)
	}
}
