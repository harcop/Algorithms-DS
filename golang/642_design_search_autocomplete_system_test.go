package leetcode

import "testing"

func TestAutocompleteSystemExample(t *testing.T) {
	sys := NewAutocompleteSystem(
		[]string{"i love you", "island", "iroman", "i love leetcode"},
		[]int{5, 3, 2, 2},
	)
	if got := sys.Input('i'); !sameStrings(got, []string{"i love you", "island", "i love leetcode"}) {
		t.Fatalf("Input(i) = %v", got)
	}
	if got := sys.Input(' '); !sameStrings(got, []string{"i love you", "i love leetcode"}) {
		t.Fatalf("Input(space) = %v", got)
	}
	if got := sys.Input('a'); len(got) != 0 {
		t.Fatalf("Input(a) = %v, want none", got)
	}
	if got := sys.Input('#'); len(got) != 0 {
		t.Fatalf("Input(#) = %v, want none", got)
	}
	if got := sys.Input('i'); !sameStrings(got, []string{"i love you", "island", "i love leetcode"}) {
		t.Fatalf("Input(i) after save = %v", got)
	}
}
