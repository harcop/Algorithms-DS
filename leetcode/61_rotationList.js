/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */
var rotateRight = function(head, k) {
    const hash = {} // could be array also
    let current = head;
    let length = 0;
    while(current !== null) {
        length++;
        hash[length] = current;
        current = current.next;
    }
    let rotation = k % length; // 1
    let endIndex = length - rotation; // 3
    let newHead = null
    if(rotation > 0) {
        newHead = hash[endIndex].next;
        hash[endIndex].next = null;
        hash[length].next = head
        return newHead;
    } else {
        return head
    }
};
