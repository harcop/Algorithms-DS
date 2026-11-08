package leetcode

import "testing"

func TestSequenceReconstructionExamples(t *testing.T) {
	if sequenceReconstruction([]int{1, 2, 3}, [][]int{{1, 2}, {1, 3}}) {
		t.Fatal("sequenceReconstruction(missing edge) = true, want false")
	}
	if sequenceReconstruction([]int{1, 2, 3}, [][]int{{1, 2}}) {
		t.Fatal("sequenceReconstruction(short seqs) = true, want false")
	}
	if !sequenceReconstruction([]int{1, 2, 3}, [][]int{{1, 2}, {1, 3}, {2, 3}}) {
		t.Fatal("sequenceReconstruction(unique order) = false, want true")
	}
}
