package leetcode

import "testing"

func TestPrintTreeExamples(t *testing.T) {
	got := printTree(makeTree(1, makeTree(2, nil, nil), nil))
	want := [][]string{{"", "1", ""}, {"2", "", ""}}
	if !sameStringGrid(got, want) {
		t.Fatalf("printTree() = %v, want %v", got, want)
	}

	got = printTree(makeTree(1,
		makeTree(2, nil, makeTree(4, nil, nil)),
		makeTree(3, nil, nil),
	))
	want = [][]string{
		{"", "", "", "1", "", "", ""},
		{"", "2", "", "", "", "3", ""},
		{"", "", "4", "", "", "", ""},
	}
	if !sameStringGrid(got, want) {
		t.Fatalf("printTree() = %v, want %v", got, want)
	}
}

func sameStringGrid(a, b [][]string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if !sameStrings(a[i], b[i]) {
			return false
		}
	}
	return true
}
