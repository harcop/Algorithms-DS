package leetcode

// LeetCode #170 - Two Sum III - Data structure design
//
// Count each added number, then look for a complement that is already stored.
type TwoSum struct {
	count map[int]int
}

func NewTwoSum() TwoSum {
	return TwoSum{count: map[int]int{}}
}

func (store *TwoSum) Add(number int) {
	store.count[number]++
}

func (store *TwoSum) Find(value int) bool {
	for number, count := range store.count {
		need := value - number
		if need == number {
			if count > 1 {
				return true
			}
			continue
		}
		if store.count[need] > 0 {
			return true
		}
	}
	return false
}
