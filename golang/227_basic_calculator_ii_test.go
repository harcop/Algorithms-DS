package leetcode

import "testing"

func TestCalculateIIExampleOne(t *testing.T) {
	if got := calculateII("3+2*2"); got != 7 {
		t.Fatalf("calculateII() = %d, want 7", got)
	}
}

func TestCalculateIIExampleTwo(t *testing.T) {
	if got := calculateII(" 3/2 "); got != 1 {
		t.Fatalf("calculateII() = %d, want 1", got)
	}
}

func TestCalculateIIExampleThree(t *testing.T) {
	if got := calculateII(" 3+5 / 2 "); got != 5 {
		t.Fatalf("calculateII() = %d, want 5", got)
	}
}
