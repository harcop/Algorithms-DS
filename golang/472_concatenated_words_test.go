package leetcode

import "testing"

func TestFindAllConcatenatedWordsExample(t *testing.T) {
	words := []string{"cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat"}
	got := findAllConcatenatedWordsInADict(words)
	want := []string{"catsdogcats", "dogcatsdog", "ratcatdogcat"}
	if !sameStrings(got, want) {
		t.Fatalf("findAllConcatenatedWordsInADict() = %v, want %v", got, want)
	}
}
