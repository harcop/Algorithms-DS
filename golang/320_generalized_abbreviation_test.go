package leetcode

import "testing"

func TestGenerateAbbreviationsExampleWord(t *testing.T) {
	got := generateAbbreviations("word")
	if len(got) != 16 {
		t.Fatalf("generateAbbreviations() has %d results, want 16", len(got))
	}
	hasWord, hasFour := false, false
	for _, abbr := range got {
		if abbr == "word" {
			hasWord = true
		}
		if abbr == "4" {
			hasFour = true
		}
	}
	if !hasWord || !hasFour {
		t.Fatalf("generateAbbreviations() = %v, want word and 4", got)
	}
}
