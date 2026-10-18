package leetcode

import "testing"

func TestCopyRandomListExampleOne(t *testing.T) {
	a := &RandomNode{Val: 7}
	b := &RandomNode{Val: 13}
	c := &RandomNode{Val: 11}
	d := &RandomNode{Val: 10}
	e := &RandomNode{Val: 1}
	a.Next, b.Next, c.Next, d.Next = b, c, d, e
	b.Random, c.Random, d.Random, e.Random = a, e, c, a

	copied := copyRandomList(a)
	if copied == nil || copied == a || copied.Val != 7 || copied.Next == nil || copied.Next == b || copied.Next.Val != 13 {
		t.Fatalf("copyRandomList() = %+v, want a new list starting 7, 13", copied)
	}
	if copied.Next.Random != copied {
		t.Fatal("copyRandomList() random link does not point at the copied node")
	}
}
