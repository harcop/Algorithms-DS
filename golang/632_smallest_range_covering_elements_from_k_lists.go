package leetcode

import (
	"container/heap"
	"math"
)

// LeetCode #632 - Smallest Range Covering Elements from K Lists
//
// One value from each list sits in a heap, and the window moves forward from the smallest.
func smallestRange(nums [][]int) []int {
	heads := &rangeHeadHeap{}
	highest := math.MinInt
	for list, row := range nums {
		heap.Push(heads, rangeHead{val: row[0], list: list, index: 0})
		if row[0] > highest {
			highest = row[0]
		}
	}
	bestLow, bestHigh := 0, math.MaxInt
	for {
		low := heap.Pop(heads).(rangeHead)
		if highest-low.val < bestHigh-bestLow {
			bestLow, bestHigh = low.val, highest
		}
		if low.index+1 == len(nums[low.list]) {
			break
		}
		next := nums[low.list][low.index+1]
		if next > highest {
			highest = next
		}
		heap.Push(heads, rangeHead{val: next, list: low.list, index: low.index + 1})
	}
	return []int{bestLow, bestHigh}
}

type rangeHead struct {
	val, list, index int
}

type rangeHeadHeap []rangeHead

func (h rangeHeadHeap) Len() int           { return len(h) }
func (h rangeHeadHeap) Less(i, j int) bool { return h[i].val < h[j].val }
func (h rangeHeadHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *rangeHeadHeap) Push(x any) { *h = append(*h, x.(rangeHead)) }

func (h *rangeHeadHeap) Pop() any {
	old := *h
	head := old[len(old)-1]
	*h = old[:len(old)-1]
	return head
}
