package leetcode

import (
	"container/heap"
	"sort"
)

// LeetCode #502 - IPO
//
// Projects are unlocked by current capital, and a max-heap always takes the largest available profit.
func findMaximizedCapital(k int, w int, profits []int, capital []int) int {
	projects := make([][2]int, len(profits))
	for i := range profits {
		projects[i] = [2]int{capital[i], profits[i]}
	}
	sort.Slice(projects, func(i, j int) bool {
		return projects[i][0] < projects[j][0]
	})
	available := &ipoProfitHeap{}
	next := 0
	for done := 0; done < k; done++ {
		for next < len(projects) && projects[next][0] <= w {
			heap.Push(available, projects[next][1])
			next++
		}
		if available.Len() == 0 {
			break
		}
		w += heap.Pop(available).(int)
	}
	return w
}

type ipoProfitHeap []int

func (h ipoProfitHeap) Len() int           { return len(h) }
func (h ipoProfitHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h ipoProfitHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *ipoProfitHeap) Push(x any) { *h = append(*h, x.(int)) }

func (h *ipoProfitHeap) Pop() any {
	old := *h
	profit := old[len(old)-1]
	*h = old[:len(old)-1]
	return profit
}
