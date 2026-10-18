package leetcode

// LRUCache keeps the most recently used keys and drops the least recent one past capacity.
// LeetCode names the constructor Constructor.
type LRUCache struct {
	capacity int
	items    map[int]*lruEntry
	head     *lruEntry
	tail     *lruEntry
}

type lruEntry struct {
	key, value int
	prev, next *lruEntry
}

func NewLRUCache(capacity int) LRUCache {
	head, tail := &lruEntry{}, &lruEntry{}
	head.next = tail
	tail.prev = head
	return LRUCache{
		capacity: capacity,
		items:    make(map[int]*lruEntry, capacity),
		head:     head,
		tail:     tail,
	}
}

func (cache *LRUCache) Get(key int) int {
	entry, ok := cache.items[key]
	if !ok {
		return -1
	}
	cache.moveToFront(entry)
	return entry.value
}

func (cache *LRUCache) Put(key int, value int) {
	if entry, ok := cache.items[key]; ok {
		entry.value = value
		cache.moveToFront(entry)
		return
	}
	if len(cache.items) == cache.capacity {
		oldest := cache.tail.prev
		cache.unlink(oldest)
		delete(cache.items, oldest.key)
	}
	entry := &lruEntry{key: key, value: value}
	cache.items[key] = entry
	cache.insertFront(entry)
}

func (cache *LRUCache) moveToFront(entry *lruEntry) {
	cache.unlink(entry)
	cache.insertFront(entry)
}

func (cache *LRUCache) unlink(entry *lruEntry) {
	entry.prev.next = entry.next
	entry.next.prev = entry.prev
}

func (cache *LRUCache) insertFront(entry *lruEntry) {
	entry.next = cache.head.next
	entry.prev = cache.head
	cache.head.next.prev = entry
	cache.head.next = entry
}
