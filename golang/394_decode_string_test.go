package leetcode

import "testing"

func TestDecodeStringExamples(t *testing.T) {
	if got := decodeString("3[a]2[bc]"); got != "aaabcbc" {
		t.Fatalf("decodeString() = %q, want aaabcbc", got)
	}
	if got := decodeString("3[a2[c]]"); got != "accaccacc" {
		t.Fatalf("decodeString() = %q, want accaccacc", got)
	}
	if got := decodeString("2[abc]3[cd]ef"); got != "abcabccdcdcdef" {
		t.Fatalf("decodeString() = %q, want abcabccdcdcdef", got)
	}
}
