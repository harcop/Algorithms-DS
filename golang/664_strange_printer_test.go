package leetcode

import "testing"

func TestStrangePrinterExamples(t *testing.T) {
	if got := strangePrinter("aaabbb"); got != 2 {
		t.Fatalf("strangePrinter(aaabbb) = %d, want 2", got)
	}
	if got := strangePrinter("aba"); got != 2 {
		t.Fatalf("strangePrinter(aba) = %d, want 2", got)
	}
}
