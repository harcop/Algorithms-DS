package leetcode

import "testing"

func TestSerializeBSTRoundTrip(t *testing.T) {
	root := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	data := serializeBST(root)
	if again := serializeBST(deserializeBST(data)); again != data {
		t.Fatalf("serializeBST(deserializeBST()) = %q, want %q", again, data)
	}
	if deserializeBST(serializeBST(nil)) != nil {
		t.Fatal("deserializeBST(empty) = non-nil, want nil")
	}
}
