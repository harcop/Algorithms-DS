package leetcode

import "math"

// LeetCode #465 - Optimal Account Balancing
//
// Nonzero balances are debts, and each transfer cancels one debt against an opposite balance.
func minTransfers(transactions [][]int) int {
	balance := map[int]int{}
	for _, txn := range transactions {
		balance[txn[0]] += txn[2]
		balance[txn[1]] -= txn[2]
	}
	debts := make([]int, 0, len(balance))
	for _, amount := range balance {
		if amount != 0 {
			debts = append(debts, amount)
		}
	}
	return settleTransfers(debts, 0)
}

func settleTransfers(debts []int, start int) int {
	for start < len(debts) && debts[start] == 0 {
		start++
	}
	if start == len(debts) {
		return 0
	}
	best := math.MaxInt
	for j := start + 1; j < len(debts); j++ {
		if int64(debts[j])*int64(debts[start]) >= 0 {
			continue
		}
		debts[j] += debts[start]
		best = min(best, 1+settleTransfers(debts, start+1))
		debts[j] -= debts[start]
	}
	if best == math.MaxInt {
		return 0
	}
	return best
}
