package leetcode

// LeetCode #362 - Design Hit Counter
//
// Hits are stored in time order and dropped once they are 300 seconds old.
type HitCounter struct {
	hits []int
}

func NewHitCounter() *HitCounter {
	return &HitCounter{}
}

func (h *HitCounter) Hit(timestamp int) {
	h.hits = append(h.hits, timestamp)
}

func (h *HitCounter) GetHits(timestamp int) int {
	cutoff := timestamp - 300
	i := 0
	for i < len(h.hits) && h.hits[i] <= cutoff {
		i++
	}
	h.hits = h.hits[i:]
	return len(h.hits)
}
