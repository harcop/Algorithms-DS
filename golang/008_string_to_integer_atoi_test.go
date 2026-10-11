package leetcode

import "testing"

func TestMyAtoiExampleOne(t *testing.T) {
	if got := myAtoi("42"); got != 42 {
		t.Fatalf("myAtoi() = %d, want 42", got)
	}
}

func TestMyAtoiExampleTwo(t *testing.T) {
	if got := myAtoi("   -42"); got != -42 {
		t.Fatalf("myAtoi() = %d, want -42", got)
	}
}

func TestMyAtoiExampleThree(t *testing.T) {
	if got := myAtoi("4193 with words"); got != 4193 {
		t.Fatalf("myAtoi() = %d, want 4193", got)
	}
}

func TestMyAtoiClampsPositive(t *testing.T) {
	if got := myAtoi("91283472332"); got != 2147483647 {
		t.Fatalf("myAtoi() = %d, want 2147483647", got)
	}
}

func TestMyAtoiClampsNegative(t *testing.T) {
	if got := myAtoi("-91283472332"); got != -2147483648 {
		t.Fatalf("myAtoi() = %d, want -2147483648", got)
	}
}
