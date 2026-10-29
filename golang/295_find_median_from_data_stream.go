package leetcode

import "container/heap"

// LeetCode #295 - Find Median from Data Stream
//
// Keep the smaller half in a max-heap and the larger half in a min-heap.
type MedianFinder struct {
	lo maxIntHeap
	hi minIntHeap
}

func NewMedianFinder() MedianFinder {
	return MedianFinder{}
}

func (finder *MedianFinder) AddNum(num int) {
	heap.Push(&finder.hi, num)
	heap.Push(&finder.lo, heap.Pop(&finder.hi))
	if finder.lo.Len() > finder.hi.Len()+1 {
		heap.Push(&finder.hi, heap.Pop(&finder.lo))
	}
}

func (finder *MedianFinder) FindMedian() float64 {
	if finder.lo.Len() > finder.hi.Len() {
		return float64(finder.lo[0])
	}
	return float64(finder.lo[0]+finder.hi[0]) / 2
}

type maxIntHeap []int

func (h maxIntHeap) Len() int           { return len(h) }
func (h maxIntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h maxIntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *maxIntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *maxIntHeap) Pop() any {
	old := *h
	value := old[len(old)-1]
	*h = old[:len(old)-1]
	return value
}

type minIntHeap []int

func (h minIntHeap) Len() int           { return len(h) }
func (h minIntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h minIntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minIntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *minIntHeap) Pop() any {
	old := *h
	value := old[len(old)-1]
	*h = old[:len(old)-1]
	return value
}
