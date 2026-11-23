package leetcode

import "sort"

// LeetCode #642 - Design Search Autocomplete System
//
// A trie keeps each historical sentence, and each typed character returns the three hottest matches.
type autoNode struct {
	children map[byte]*autoNode
	sentence string
	times    int
}

type AutocompleteSystem struct {
	root    *autoNode
	node    *autoNode
	current []byte
	dead    bool
}

func NewAutocompleteSystem(sentences []string, times []int) *AutocompleteSystem {
	root := &autoNode{children: map[byte]*autoNode{}}
	sys := &AutocompleteSystem{root: root, node: root}
	for i, sentence := range sentences {
		sys.add(sentence, times[i])
	}
	return sys
}

func (a *AutocompleteSystem) add(sentence string, times int) {
	node := a.root
	for i := 0; i < len(sentence); i++ {
		child := node.children[sentence[i]]
		if child == nil {
			child = &autoNode{children: map[byte]*autoNode{}}
			node.children[sentence[i]] = child
		}
		node = child
	}
	node.times += times
	node.sentence = sentence
}

func (a *AutocompleteSystem) Input(c byte) []string {
	if c == '#' {
		a.add(string(a.current), 1)
		a.current = nil
		a.node = a.root
		a.dead = false
		return nil
	}
	a.current = append(a.current, c)
	if a.dead {
		return nil
	}
	next := a.node.children[c]
	if next == nil {
		a.dead = true
		return nil
	}
	a.node = next
	type hit struct {
		sentence string
		times    int
	}
	var hits []hit
	var walk func(*autoNode)
	walk = func(node *autoNode) {
		if node.times > 0 {
			hits = append(hits, hit{node.sentence, node.times})
		}
		for _, child := range node.children {
			walk(child)
		}
	}
	walk(a.node)
	sort.Slice(hits, func(i, j int) bool {
		if hits[i].times == hits[j].times {
			return hits[i].sentence < hits[j].sentence
		}
		return hits[i].times > hits[j].times
	})
	if len(hits) > 3 {
		hits = hits[:3]
	}
	out := make([]string, len(hits))
	for i, item := range hits {
		out[i] = item.sentence
	}
	return out
}
