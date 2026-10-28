package leetcode

import "testing"

func TestValidWordAbbrExample(t *testing.T) {
	dict := NewValidWordAbbr([]string{"deer", "door", "cake", "card"})
	if dict.IsUnique("dear") {
		t.Fatal("IsUnique(dear) = true, want false")
	}
	if !dict.IsUnique("cart") {
		t.Fatal("IsUnique(cart) = false, want true")
	}
	if dict.IsUnique("cane") {
		t.Fatal("IsUnique(cane) = true, want false")
	}
	if !dict.IsUnique("make") {
		t.Fatal("IsUnique(make) = false, want true")
	}
}
