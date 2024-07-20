/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function(head) {
    let current = head;
    let seen = []
    while(current !== null) {
        let currentValue = current.val;
        let currentNext = current.next;
        let i = 0;
        while(currentNext && currentNext.val === currentValue) {
            currentNext = currentNext.next;
            current.next = currentNext;
            i++
        }
        if(i > 0) {
            seen.push(current.val)
        }
        current = current.next
    }
    current = head;
    while(current !== null) {
        if(seen.includes(head.val)) {
            head = head.next
            current = head;
        } else {
            if(current.next && seen.includes(current.next.val)) {
                current.next = current.next.next
            } else {
                current = current.next
            }
        }
    }
    return head
};
