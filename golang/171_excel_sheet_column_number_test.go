package leetcode

import "testing"

func TestTitleToNumberExampleOne(t *testing.T) {
	if got := titleToNumber("A"); got != 1 {
		t.Fatalf("titleToNumber() = %d, want 1", got)
	}
}

func TestTitleToNumberExampleTwo(t *testing.T) {
	if got := titleToNumber("AB"); got != 28 {
		t.Fatalf("titleToNumber() = %d, want 28", got)
	}
}

func TestTitleToNumberExampleThree(t *testing.T) {
	if got := titleToNumber("ZY"); got != 701 {
		t.Fatalf("titleToNumber() = %d, want 701", got)
	}
}
