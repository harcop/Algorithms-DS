package leetcode

import "testing"

func TestSerializeRoundTrip(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	encoded := serialize(root)
	if got := serialize(deserialize(encoded)); got != encoded {
		t.Fatalf("serialize(deserialize()) = %q, want %q", got, encoded)
	}
}
