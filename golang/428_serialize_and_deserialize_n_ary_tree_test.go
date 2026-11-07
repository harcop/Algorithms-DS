package leetcode

import "testing"

func TestSerializeNaryRoundTrip(t *testing.T) {
	root := sampleNary()
	encoded := serializeNary(root)
	if got := serializeNary(deserializeNary(encoded)); got != encoded {
		t.Fatalf("serializeNary(deserializeNary()) = %q, want %q", got, encoded)
	}
	if deserializeNary("") != nil {
		t.Fatal("deserializeNary(empty) != nil")
	}
}

func sampleNary() *NaryNode {
	return &NaryNode{
		Val: 1,
		Children: []*NaryNode{
			{Val: 3, Children: []*NaryNode{{Val: 5}, {Val: 6}}},
			{Val: 2},
			{Val: 4},
		},
	}
}
