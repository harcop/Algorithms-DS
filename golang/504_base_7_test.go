package leetcode

import "testing"

func TestConvertToBase7Examples(t *testing.T) {
	if got := convertToBase7(100); got != "202" {
		t.Fatalf("convertToBase7(100) = %q, want 202", got)
	}
	if got := convertToBase7(-7); got != "-10" {
		t.Fatalf("convertToBase7(-7) = %q, want -10", got)
	}
	if got := convertToBase7(0); got != "0" {
		t.Fatalf("convertToBase7(0) = %q, want 0", got)
	}
}
