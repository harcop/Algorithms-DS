package leetcode

import "testing"

func TestSmallestGoodBaseExamples(t *testing.T) {
	if got := smallestGoodBase("13"); got != "3" {
		t.Fatalf("smallestGoodBase(13) = %q, want 3", got)
	}
	if got := smallestGoodBase("4681"); got != "8" {
		t.Fatalf("smallestGoodBase(4681) = %q, want 8", got)
	}
	if got := smallestGoodBase("1000000000000000000"); got != "999999999999999999" {
		t.Fatalf("smallestGoodBase(10^18) = %q, want 999999999999999999", got)
	}
}
