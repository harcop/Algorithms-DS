package leetcode

// LeetCode #460 - LFU Cache
//
// Keys of the same frequency stay in least-recently-used order, and a new key evicts the oldest key at the lowest frequency.
type LFUCache struct {
	capacity int
	minFreq  int
	values   map[int]int
	freq     map[int]int
	buckets  map[int][]int
}

func NewLFUCache(capacity int) *LFUCache {
	return &LFUCache{
		capacity: capacity,
		values:   map[int]int{},
		freq:     map[int]int{},
		buckets:  map[int][]int{},
	}
}

func (c *LFUCache) Get(key int) int {
	if _, ok := c.values[key]; !ok {
		return -1
	}
	c.touch(key)
	return c.values[key]
}

func (c *LFUCache) Put(key int, value int) {
	if c.capacity == 0 {
		return
	}
	if _, ok := c.values[key]; ok {
		c.values[key] = value
		c.touch(key)
		return
	}
	if len(c.values) == c.capacity {
		bucket := c.buckets[c.minFreq]
		victim := bucket[0]
		c.buckets[c.minFreq] = bucket[1:]
		if len(c.buckets[c.minFreq]) == 0 {
			delete(c.buckets, c.minFreq)
		}
		delete(c.values, victim)
		delete(c.freq, victim)
	}
	c.values[key] = value
	c.freq[key] = 1
	c.buckets[1] = append(c.buckets[1], key)
	c.minFreq = 1
}

func (c *LFUCache) touch(key int) {
	freq := c.freq[key]
	c.detach(key, freq)
	freq++
	c.freq[key] = freq
	c.buckets[freq] = append(c.buckets[freq], key)
}

func (c *LFUCache) detach(key, freq int) {
	bucket := c.buckets[freq]
	for i, stored := range bucket {
		if stored != key {
			continue
		}
		c.buckets[freq] = append(bucket[:i], bucket[i+1:]...)
		break
	}
	if len(c.buckets[freq]) == 0 {
		delete(c.buckets, freq)
		if freq == c.minFreq {
			c.minFreq++
		}
	}
}
