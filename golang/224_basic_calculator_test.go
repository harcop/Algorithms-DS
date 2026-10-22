package leetcode

import "testing"

func TestCalculateExampleOne(t *testing.T) {
	if got := calculate("1 + 1"); got != 2 {
		t.Fatalf("calculate() = %d, want 2", got)
	}
}

func TestCalculateExampleTwo(t *testing.T) {
	if got := calculate(" 2-1 + 2 "); got != 3 {
		t.Fatalf("calculate() = %d, want 3", got)
	}
}

func TestCalculateExampleThree(t *testing.T) {
	if got := calculate("(1+(4+5+2)-3)+(6+8)"); got != 23 {
		t.Fatalf("calculate() = %d, want 23", got)
	}
}
