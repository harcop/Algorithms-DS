package leetcode

import "testing"

func TestExcelSumFormulaExample(t *testing.T) {
	sheet := NewExcel(3, 'C')
	if got := sheet.Get(1, 'A'); got != 0 {
		t.Fatalf("Get(1, A) = %d, want 0", got)
	}
	sheet.Set(1, 'A', 2)
	if got := sheet.Sum(3, 'C', []string{"A1", "A1:B2"}); got != 4 {
		t.Fatalf("Sum(3, C) = %d, want 4", got)
	}
	sheet.Set(2, 'B', 2)
	if got := sheet.Get(3, 'C'); got != 6 {
		t.Fatalf("Get(3, C) = %d, want 6", got)
	}
	sheet.Set(3, 'C', 9)
	if got := sheet.Get(3, 'C'); got != 9 {
		t.Fatalf("Get(3, C) after Set = %d, want 9", got)
	}
}

func TestExcelSumFollowsOtherFormulas(t *testing.T) {
	sheet := NewExcel(1, 'C')
	sheet.Set(1, 'A', 1)
	if got := sheet.Sum(1, 'B', []string{"A1"}); got != 1 {
		t.Fatalf("Sum(1, B) = %d, want 1", got)
	}
	if got := sheet.Sum(1, 'C', []string{"B1"}); got != 1 {
		t.Fatalf("Sum(1, C) = %d, want 1", got)
	}
	sheet.Set(1, 'A', 4)
	if got := sheet.Get(1, 'C'); got != 4 {
		t.Fatalf("Get(1, C) = %d, want 4", got)
	}
}
