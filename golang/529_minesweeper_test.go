package leetcode

import "testing"

func TestUpdateBoardExamples(t *testing.T) {
	board := [][]byte{
		{'E', 'E', 'E', 'E', 'E'},
		{'E', 'E', 'M', 'E', 'E'},
		{'E', 'E', 'E', 'E', 'E'},
		{'E', 'E', 'E', 'E', 'E'},
	}
	got := updateBoard(board, []int{3, 0})
	want := [][]byte{
		{'B', '1', 'E', '1', 'B'},
		{'B', '1', 'M', '1', 'B'},
		{'B', '1', '1', '1', 'B'},
		{'B', 'B', 'B', 'B', 'B'},
	}
	if !sameBoard(got, want) {
		t.Fatalf("updateBoard() = %q, want the revealed board", got)
	}
	revealed := [][]byte{
		{'B', '1', 'E', '1', 'B'},
		{'B', '1', 'M', '1', 'B'},
		{'B', '1', '1', '1', 'B'},
		{'B', 'B', 'B', 'B', 'B'},
	}
	got = updateBoard(revealed, []int{1, 2})
	want = [][]byte{
		{'B', '1', 'E', '1', 'B'},
		{'B', '1', 'X', '1', 'B'},
		{'B', '1', '1', '1', 'B'},
		{'B', 'B', 'B', 'B', 'B'},
	}
	if !sameBoard(got, want) {
		t.Fatalf("updateBoard() = %q, want the mine marked", got)
	}
}

func sameBoard(a, b [][]byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if !sameBytes(a[i], b[i]) {
			return false
		}
	}
	return true
}
