package leetcode

import "testing"

func TestComplexNumberMultiplyExamples(t *testing.T) {
	if got := complexNumberMultiply("1+1i", "1+1i"); got != "0+2i" {
		t.Fatalf("complexNumberMultiply() = %q, want 0+2i", got)
	}
	if got := complexNumberMultiply("1+-1i", "1+-1i"); got != "0+-2i" {
		t.Fatalf("complexNumberMultiply() = %q, want 0+-2i", got)
	}
}
