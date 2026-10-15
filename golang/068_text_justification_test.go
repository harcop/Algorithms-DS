package leetcode

import "testing"

func TestFullJustifyExampleOne(t *testing.T) {
	got := fullJustify([]string{"This", "is", "an", "example", "of", "text", "justification."}, 16)
	want := []string{
		"This    is    an",
		"example  of text",
		"justification.  ",
	}
	if len(got) != len(want) {
		t.Fatalf("fullJustify() = %q, want %q", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("fullJustify()[%d] = %q, want %q", i, got[i], want[i])
		}
	}
}
