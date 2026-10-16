package leetcode

import "testing"

func TestExistExampleOne(t *testing.T) {
	board := [][]byte{[]byte("ABCE"), []byte("SFCS"), []byte("ADEE")}
	if !exist(board, "ABCCED") {
		t.Fatal("exist() = false, want true")
	}
}

func TestExistExampleTwo(t *testing.T) {
	board := [][]byte{[]byte("ABCE"), []byte("SFCS"), []byte("ADEE")}
	if !exist(board, "SEE") {
		t.Fatal("exist() = false, want true")
	}
}

func TestExistExampleThree(t *testing.T) {
	board := [][]byte{[]byte("ABCE"), []byte("SFCS"), []byte("ADEE")}
	if exist(board, "ABCB") {
		t.Fatal("exist() = true, want false")
	}
}
