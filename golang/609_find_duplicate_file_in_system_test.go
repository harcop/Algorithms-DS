package leetcode

import (
	"sort"
	"testing"
)

func TestFindDuplicateFilesExample(t *testing.T) {
	paths := []string{
		"root/a 1.txt(abcd) 2.txt(efgh)",
		"root/c 3.txt(abcd)",
		"root/c/d 4.txt(efgh)",
		"root 4.txt(efgh)",
	}
	got := normalizeFileGroups(findDuplicateFiles(paths))
	want := normalizeFileGroups([][]string{
		{"root/a/2.txt", "root/c/d/4.txt", "root/4.txt"},
		{"root/a/1.txt", "root/c/3.txt"},
	})
	if !sameFileGroups(got, want) {
		t.Fatalf("findDuplicateFiles() = %v, want %v", got, want)
	}

	unique := []string{
		"root/a 1.txt(abcd) 2.txt(efsfgh)",
		"root/c 3.txt(abdfcd)",
		"root/c/d 4.txt(efggdfh)",
	}
	if got := findDuplicateFiles(unique); len(got) != 0 {
		t.Fatalf("findDuplicateFiles() = %v, want none", got)
	}
}

func normalizeFileGroups(groups [][]string) [][]string {
	out := make([][]string, len(groups))
	for i, group := range groups {
		files := append([]string(nil), group...)
		sort.Strings(files)
		out[i] = files
	}
	sort.Slice(out, func(i, j int) bool {
		return out[i][0] < out[j][0]
	})
	return out
}

func sameFileGroups(a, b [][]string) bool {
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
