package leetcode

// LeetCode #677 - Map Sum Pairs
//
// Each prefix node stores the total of the keys below it, and a repeated key replaces its old value.
type mapSumNode struct {
	children [26]*mapSumNode
	sum      int
}

type MapSum struct {
	root   *mapSumNode
	values map[string]int
}

func NewMapSum() *MapSum {
	return &MapSum{root: &mapSumNode{}, values: map[string]int{}}
}

func (m *MapSum) Insert(key string, val int) {
	delta := val - m.values[key]
	m.values[key] = val
	node := m.root
	for i := 0; i < len(key); i++ {
		index := key[i] - 'a'
		if node.children[index] == nil {
			node.children[index] = &mapSumNode{}
		}
		node = node.children[index]
		node.sum += delta
	}
}

func (m *MapSum) Sum(prefix string) int {
	node := m.root
	for i := 0; i < len(prefix); i++ {
		node = node.children[prefix[i]-'a']
		if node == nil {
			return 0
		}
	}
	return node.sum
}
