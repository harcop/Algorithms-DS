package leetcode

import "testing"

func TestStrongPasswordCheckerExamples(t *testing.T) {
	if got := strongPasswordChecker("a"); got != 5 {
		t.Fatalf("strongPasswordChecker(a) = %d, want 5", got)
	}
	if got := strongPasswordChecker("aA1"); got != 3 {
		t.Fatalf("strongPasswordChecker(aA1) = %d, want 3", got)
	}
	if got := strongPasswordChecker("1337C0d3"); got != 0 {
		t.Fatalf("strongPasswordChecker(1337C0d3) = %d, want 0", got)
	}
	if got := strongPasswordChecker("aaa111"); got != 2 {
		t.Fatalf("strongPasswordChecker(aaa111) = %d, want 2", got)
	}
	if got := strongPasswordChecker("aaaaaaaaaaaaaaaaaaaaa"); got != 7 {
		t.Fatalf("strongPasswordChecker(21 a's) = %d, want 7", got)
	}
}
