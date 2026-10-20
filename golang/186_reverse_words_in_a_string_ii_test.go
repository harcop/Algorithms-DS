package leetcode

import "testing"

func TestReverseWordsIIExample(t *testing.T) {
	s := []byte("the sky is blue")
	reverseWordsII(s)
	if got := string(s); got != "blue is sky the" {
		t.Fatalf("reverseWordsII() = %q, want %q", got, "blue is sky the")
	}
}
