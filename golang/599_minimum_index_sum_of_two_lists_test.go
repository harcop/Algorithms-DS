package leetcode

import "testing"

func TestFindRestaurantExamples(t *testing.T) {
	list1 := []string{"Shogun", "Tapioca Express", "Burger King", "KFC"}
	got := findRestaurant(list1, []string{"Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"})
	if !sameStrings(got, []string{"Shogun"}) {
		t.Fatalf("findRestaurant = %v, want [Shogun]", got)
	}
	got = findRestaurant([]string{"happy", "sad", "good"}, []string{"sad", "happy", "good"})
	if !sameStrings(got, []string{"sad", "happy"}) {
		t.Fatalf("findRestaurant = %v, want [sad happy]", got)
	}
}
