package leetcode

import "testing"

func TestEncodeNaryRoundTrip(t *testing.T) {
	root := sampleNary()
	encoded := encodeNary(root)
	if got := serializeNary(decodeNary(encoded)); got != serializeNary(root) {
		t.Fatalf("decodeNary(encodeNary()) = %q, want %q", got, serializeNary(root))
	}
	if encodeNary(nil) != nil || decodeNary(nil) != nil {
		t.Fatal("nil n-ary tree did not stay nil")
	}
}
