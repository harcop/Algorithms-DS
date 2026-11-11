package leetcode

import "sort"

// LeetCode #497 - Random Point in Non-overlapping Rectangles
//
// Prefix areas pick a rectangle in proportion to how many points it contains, then a point is chosen inside it.
type RandomRect struct {
	rects  [][]int
	prefix []int64
	rng    uint64
}

func NewRandomRect(rects [][]int) *RandomRect {
	prefix := make([]int64, len(rects))
	var total int64
	for i, rect := range rects {
		width := int64(rect[2] - rect[0] + 1)
		height := int64(rect[3] - rect[1] + 1)
		total += width * height
		prefix[i] = total
	}
	return &RandomRect{rects: rects, prefix: prefix, rng: 0x9e3779b97f4a7c15}
}

func (r *RandomRect) Pick() []int {
	total := r.prefix[len(r.prefix)-1]
	target := int64(r.nextUint()%uint64(total)) + 1
	index := sort.Search(len(r.prefix), func(i int) bool { return r.prefix[i] >= target })
	rect := r.rects[index]
	width := uint64(rect[2] - rect[0] + 1)
	height := uint64(rect[3] - rect[1] + 1)
	x := rect[0] + int(r.nextUint()%width)
	y := rect[1] + int(r.nextUint()%height)
	return []int{x, y}
}

func (r *RandomRect) nextUint() uint64 {
	r.rng ^= r.rng << 7
	r.rng ^= r.rng >> 9
	return r.rng
}
