package leetcode

import "testing"

func TestOriginalDigitsExamples(t *testing.T) {
	if got := originalDigits("owoztneoer"); got != "012" {
		t.Fatalf("originalDigits(owoztneoer) = %q, want 012", got)
	}
	if got := originalDigits("fviefuro"); got != "45" {
		t.Fatalf("originalDigits(fviefuro) = %q, want 45", got)
	}
}
