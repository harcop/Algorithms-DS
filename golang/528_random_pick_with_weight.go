package leetcode

// LeetCode #528 - Random Pick with Weight
//
// Prefix sums turn a random offset into the index of the weight range that contains it.
type WeightedPicker struct {
	prefix []int
	total  int
	rng    uint64
}

func NewWeightedPicker(weights []int) *WeightedPicker {
	prefix := make([]int, len(weights))
	sum := 0
	for i, weight := range weights {
		sum += weight
		prefix[i] = sum
	}
	return &WeightedPicker{prefix: prefix, total: sum, rng: 0x9e3779b97f4a7c15}
}

func (p *WeightedPicker) PickIndex() int {
	target := int(p.nextRand()%uint64(p.total)) + 1
	lo, hi := 0, len(p.prefix)
	for lo < hi {
		mid := lo + (hi-lo)/2
		if p.prefix[mid] < target {
			lo = mid + 1
			continue
		}
		hi = mid
	}
	return lo
}

func (p *WeightedPicker) nextRand() uint64 {
	p.rng = p.rng*6364136223846793005 + 1
	return p.rng
}
