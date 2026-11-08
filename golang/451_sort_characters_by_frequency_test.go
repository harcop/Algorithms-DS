package leetcode

import "testing"

func TestFrequencySortExample(t *testing.T) {
	if got := frequencySort("tree"); got != "eert" {
		t.Fatalf("frequencySort() = %q, want eert", got)
	}
	if got := frequencySort("Aabb"); got != "bbAa" {
		t.Fatalf("frequencySort() = %q, want bbAa", got)
	}
}
