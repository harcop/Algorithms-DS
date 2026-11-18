package leetcode

import "testing"

func TestIsValidTagExamples(t *testing.T) {
	if !isValidTag("<DIV>This is the first line <![CDATA[<div>]]></DIV>") {
		t.Fatal("isValidTag(cdata) = false, want true")
	}
	if !isValidTag("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>") {
		t.Fatal("isValidTag(nested cdata text) = false, want true")
	}
	if isValidTag("<A>  <B> </A>   </B>") {
		t.Fatal("isValidTag(crossed tags) = true, want false")
	}
	if isValidTag("<DIV>  unmatched <  </DIV>") {
		t.Fatal("isValidTag(unmatched <) = true, want false")
	}
	if isValidTag("<A></A><B></B>") {
		t.Fatal("isValidTag(two roots) = true, want false")
	}
}
