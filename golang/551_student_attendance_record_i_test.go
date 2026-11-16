package leetcode

import "testing"

func TestCheckRecordExamples(t *testing.T) {
	if !checkRecord("PPALLP") {
		t.Fatal("checkRecord(PPALLP) = false, want true")
	}
	if checkRecord("PPALLL") {
		t.Fatal("checkRecord(PPALLL) = true, want false")
	}
}
