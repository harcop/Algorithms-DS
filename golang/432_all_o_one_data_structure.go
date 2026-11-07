package leetcode

// LeetCode #432 - All O`one Data Structure
//
// Keys sit in frequency buckets on a doubly linked list, so the least and most frequent stay at the ends.
type AllOne struct {
	bucket map[string]*allOneBucket
	head   *allOneBucket
	tail   *allOneBucket
}

type allOneBucket struct {
	freq int
	keys map[string]struct{}
	prev *allOneBucket
	next *allOneBucket
}

func NewAllOne() *AllOne {
	head, tail := &allOneBucket{}, &allOneBucket{}
	head.next = tail
	tail.prev = head
	return &AllOne{
		bucket: map[string]*allOneBucket{},
		head:   head,
		tail:   tail,
	}
}

func (a *AllOne) Inc(key string) {
	cur := a.bucket[key]
	freq := 1
	after := a.head
	if cur != nil {
		freq = cur.freq + 1
		after = cur
		delete(cur.keys, key)
	}
	if after.next.freq == freq {
		after.next.keys[key] = struct{}{}
		a.bucket[key] = after.next
	} else {
		a.bucket[key] = a.addAfter(after, freq, key)
	}
	if cur != nil {
		a.drop(cur)
	}
}

func (a *AllOne) Dec(key string) {
	cur := a.bucket[key]
	delete(cur.keys, key)
	if cur.freq == 1 {
		delete(a.bucket, key)
		a.drop(cur)
		return
	}
	freq := cur.freq - 1
	if cur.prev.freq == freq {
		cur.prev.keys[key] = struct{}{}
		a.bucket[key] = cur.prev
	} else {
		a.bucket[key] = a.addAfter(cur.prev, freq, key)
	}
	a.drop(cur)
}

func (a *AllOne) GetMaxKey() string {
	if a.tail.prev == a.head {
		return ""
	}
	for key := range a.tail.prev.keys {
		return key
	}
	return ""
}

func (a *AllOne) GetMinKey() string {
	if a.head.next == a.tail {
		return ""
	}
	for key := range a.head.next.keys {
		return key
	}
	return ""
}

func (a *AllOne) addAfter(prev *allOneBucket, freq int, key string) *allOneBucket {
	bucket := &allOneBucket{freq: freq, keys: map[string]struct{}{key: {}}}
	next := prev.next
	prev.next = bucket
	bucket.prev = prev
	bucket.next = next
	next.prev = bucket
	return bucket
}

func (a *AllOne) drop(bucket *allOneBucket) {
	if len(bucket.keys) > 0 {
		return
	}
	bucket.prev.next = bucket.next
	bucket.next.prev = bucket.prev
}
