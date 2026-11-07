package leetcode

import "testing"

func TestFlattenMultilevelExample(t *testing.T) {
	three := &MultiNode{Val: 3}
	two := &MultiNode{Val: 2, Child: three}
	four := &MultiNode{Val: 4}
	two.Next = four
	four.Prev = two
	head := &MultiNode{Val: 1, Child: two}

	got := multilevelValues(t, flattenMultilevel(head))
	if !sameInts(got, []int{1, 2, 3, 4}) {
		t.Fatalf("flattenMultilevel() = %v, want [1 2 3 4]", got)
	}
	if flattenMultilevel(nil) != nil {
		t.Fatal("flattenMultilevel(nil) != nil")
	}
}

func multilevelValues(t *testing.T, head *MultiNode) []int {
	t.Helper()
	var out []int
	var prev *MultiNode
	for head != nil {
		if head.Child != nil {
			t.Fatalf("child remains on %d", head.Val)
		}
		if head.Prev != prev {
			t.Fatalf("prev of %d is wrong", head.Val)
		}
		out = append(out, head.Val)
		prev = head
		head = head.Next
		if len(out) > 16 {
			t.Fatal("flattened list did not end")
		}
	}
	return out
}
