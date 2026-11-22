package leetcode

import "testing"

func TestSolveEquationExamples(t *testing.T) {
	if got := solveEquation("x+5-3+x=6+x-2"); got != "x=2" {
		t.Fatalf("solveEquation() = %q, want x=2", got)
	}
	if got := solveEquation("x=x"); got != "Infinite solutions" {
		t.Fatalf("solveEquation(x=x) = %q, want Infinite solutions", got)
	}
	if got := solveEquation("2x=x"); got != "x=0" {
		t.Fatalf("solveEquation(2x=x) = %q, want x=0", got)
	}
	if got := solveEquation("x=x+2"); got != "No solution" {
		t.Fatalf("solveEquation(x=x+2) = %q, want No solution", got)
	}
	if got := solveEquation("2x+3x-6x=x+2"); got != "x=-1" {
		t.Fatalf("solveEquation() = %q, want x=-1", got)
	}
}
