package leetcode

import "testing"

func TestEvalRPNExampleOne(t *testing.T) {
	got := evalRPN([]string{"2", "1", "+", "3", "*"})
	if got != 9 {
		t.Fatalf("evalRPN() = %d, want 9", got)
	}
}

func TestEvalRPNExampleTwo(t *testing.T) {
	got := evalRPN([]string{"4", "13", "5", "/", "+"})
	if got != 6 {
		t.Fatalf("evalRPN() = %d, want 6", got)
	}
}

func TestEvalRPNExampleThree(t *testing.T) {
	got := evalRPN([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"})
	if got != 22 {
		t.Fatalf("evalRPN() = %d, want 22", got)
	}
}
