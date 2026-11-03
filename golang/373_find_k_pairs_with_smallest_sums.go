package leetcode

import "container/heap"

// LeetCode #373 - Find K Pairs with Smallest Sums
//
// A min-heap walks the next pair in each row of the sorted inputs.
func kSmallestPairs(nums1, nums2 []int, k int) [][]int {
	if len(nums1) == 0 || len(nums2) == 0 || k == 0 {
		return nil
	}
	h := &kPairHeap{}
	limit := k
	if limit > len(nums1) {
		limit = len(nums1)
	}
	for i := 0; i < limit; i++ {
		heap.Push(h, [3]int{nums1[i] + nums2[0], i, 0})
	}
	out := make([][]int, 0, k)
	for h.Len() > 0 && len(out) < k {
		item := heap.Pop(h).([3]int)
		i, j := item[1], item[2]
		out = append(out, []int{nums1[i], nums2[j]})
		if j+1 < len(nums2) {
			heap.Push(h, [3]int{nums1[i] + nums2[j+1], i, j + 1})
		}
	}
	return out
}

type kPairHeap [][3]int

func (h kPairHeap) Len() int           { return len(h) }
func (h kPairHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h kPairHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *kPairHeap) Push(x any) { *h = append(*h, x.([3]int)) }

func (h *kPairHeap) Pop() any {
	old := *h
	item := old[len(old)-1]
	*h = old[:len(old)-1]
	return item
}
