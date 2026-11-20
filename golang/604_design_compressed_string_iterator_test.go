package leetcode

import "testing"

func TestStringIteratorExample(t *testing.T) {
	it := NewStringIterator("L1e2t1C1o1d1e1")
	want := []byte{'L', 'e', 'e', 't', 'C', 'o', 'd'}
	for _, ch := range want {
		if got := it.Next(); got != ch {
			t.Fatalf("Next() = %q, want %q", got, ch)
		}
	}
	if !it.HasNext() {
		t.Fatal("HasNext() = false, want true")
	}
	if got := it.Next(); got != 'e' {
		t.Fatalf("Next() = %q, want e", got)
	}
	if it.HasNext() {
		t.Fatal("HasNext() = true, want false")
	}
	if got := it.Next(); got != ' ' {
		t.Fatalf("Next() = %q, want a space", got)
	}
}

func TestStringIteratorMultiDigitCount(t *testing.T) {
	it := NewStringIterator("a12b1")
	for i := 0; i < 12; i++ {
		if got := it.Next(); got != 'a' {
			t.Fatalf("Next() = %q, want a", got)
		}
	}
	if got := it.Next(); got != 'b' || it.HasNext() {
		t.Fatalf("Next() = %q, HasNext() = %v, want b and false", got, it.HasNext())
	}
}
