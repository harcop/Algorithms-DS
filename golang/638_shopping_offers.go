package leetcode

import "fmt"

// LeetCode #638 - Shopping Offers
//
// Each offer is applied when the remaining needs can cover it, and leftovers are bought at full price.
func shoppingOffers(price []int, special [][]int, needs []int) int {
	memo := map[string]int{}
	var spend func([]int) int
	spend = func(need []int) int {
		key := fmt.Sprint(need)
		if cached, ok := memo[key]; ok {
			return cached
		}
		best := 0
		for i, count := range need {
			best += count * price[i]
		}
		for _, offer := range special {
			next := append([]int(nil), need...)
			fits := true
			taken := false
			for i := range next {
				if offer[i] > next[i] {
					fits = false
					break
				}
				if offer[i] > 0 {
					taken = true
				}
				next[i] -= offer[i]
			}
			if !fits || !taken {
				continue
			}
			total := offer[len(offer)-1] + spend(next)
			if total < best {
				best = total
			}
		}
		memo[key] = best
		return best
	}
	return spend(needs)
}
