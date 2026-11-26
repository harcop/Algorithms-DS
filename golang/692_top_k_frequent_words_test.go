package leetcode

import "testing"

func TestTopKFrequentWordsExamples(t *testing.T) {
	got := topKFrequentWords([]string{"i", "love", "leetcode", "i", "love", "coding"}, 2)
	if !sameStrings(got, []string{"i", "love"}) {
		t.Fatalf("topKFrequentWords() = %v, want [i love]", got)
	}
	got = topKFrequentWords([]string{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"}, 4)
	if !sameStrings(got, []string{"the", "is", "sunny", "day"}) {
		t.Fatalf("topKFrequentWords() = %v, want [the is sunny day]", got)
	}
}
