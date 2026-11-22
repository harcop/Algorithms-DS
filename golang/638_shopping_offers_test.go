package leetcode

import "testing"

func TestShoppingOffersExamples(t *testing.T) {
	got := shoppingOffers([]int{2, 5}, [][]int{{3, 0, 5}, {1, 2, 10}}, []int{3, 2})
	if got != 14 {
		t.Fatalf("shoppingOffers() = %d, want 14", got)
	}
	got = shoppingOffers([]int{2, 3, 4}, [][]int{{1, 1, 0, 4}, {2, 2, 1, 9}}, []int{1, 2, 1})
	if got != 11 {
		t.Fatalf("shoppingOffers() = %d, want 11", got)
	}
}
