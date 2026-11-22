package leetcode

import (
	"container/heap"
	"sort"
)

// LeetCode #630 - Course Schedule III
//
// Courses are taken in deadline order, and a shorter course replaces the longest one when the day runs past the deadline.
func scheduleCourse(courses [][]int) int {
	sort.Slice(courses, func(i, j int) bool {
		return courses[i][1] < courses[j][1]
	})
	taken := &courseDurationHeap{}
	day := 0
	for _, course := range courses {
		duration, last := course[0], course[1]
		if day+duration <= last {
			heap.Push(taken, duration)
			day += duration
			continue
		}
		if taken.Len() > 0 && duration < (*taken)[0] {
			day += duration - (*taken)[0]
			heap.Pop(taken)
			heap.Push(taken, duration)
		}
	}
	return taken.Len()
}

type courseDurationHeap []int

func (h courseDurationHeap) Len() int           { return len(h) }
func (h courseDurationHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h courseDurationHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *courseDurationHeap) Push(x any) { *h = append(*h, x.(int)) }

func (h *courseDurationHeap) Pop() any {
	old := *h
	duration := old[len(old)-1]
	*h = old[:len(old)-1]
	return duration
}
