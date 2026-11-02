package leetcode

import "testing"

func TestMaxEnvelopesExample(t *testing.T) {
	envelopes := [][]int{{5, 4}, {6, 4}, {6, 7}, {2, 3}}
	if got := maxEnvelopes(envelopes); got != 3 {
		t.Fatalf("maxEnvelopes() = %d, want 3", got)
	}
}

func TestMaxEnvelopesSameWidth(t *testing.T) {
	envelopes := [][]int{{1, 1}, {1, 1}, {1, 1}}
	if got := maxEnvelopes(envelopes); got != 1 {
		t.Fatalf("maxEnvelopes() = %d, want 1", got)
	}
}
