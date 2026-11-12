package leetcode

import "testing"

func TestDetectCapitalUseExamples(t *testing.T) {
	if !detectCapitalUse("USA") {
		t.Fatal("detectCapitalUse(USA) = false, want true")
	}
	if !detectCapitalUse("Google") {
		t.Fatal("detectCapitalUse(Google) = false, want true")
	}
	if detectCapitalUse("FlaG") {
		t.Fatal("detectCapitalUse(FlaG) = true, want false")
	}
	if !detectCapitalUse("leetcode") {
		t.Fatal("detectCapitalUse(leetcode) = false, want true")
	}
}
