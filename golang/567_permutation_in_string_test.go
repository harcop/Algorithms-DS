package leetcode

import "testing"

func TestCheckInclusionExamples(t *testing.T) {
	if !checkInclusion("ab", "eidbaooo") {
		t.Fatal("checkInclusion(ab, eidbaooo) = false, want true")
	}
	if checkInclusion("ab", "eidboaoo") {
		t.Fatal("checkInclusion(ab, eidboaoo) = true, want false")
	}
}
