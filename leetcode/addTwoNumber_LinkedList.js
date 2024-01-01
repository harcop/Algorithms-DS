/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
    let h1 = l1
    let h2 = l2
    let add = 0;
    let rem = 0;
    let head = null
    let tempHead = null;
    while(h1 !== null && h2 !== null) {
        add = h1.val + h2.val + rem;
        rem = 0;
        if(add >= 10) {
            add = add - 10;
            rem = 1
        }
        const newNode = new ListNode(add)
        if(head) {
            head.next = newNode
            head = head.next
        } else {
            head = newNode
            tempHead = head
        }
        h1 = h1.next;
        h2 = h2.next;
    }
    while(h1) {
        add = h1.val + rem;
        rem = 0;
        if(add >= 10) {
            add = add - 10;
            rem = 1
        }
        const newNode = new ListNode(add)
        if(head) {
            head.next = newNode
            head = head.next
        } else {
            head = newNode
            tempHead = head
        }
        h1 = h1.next;
    }
    while(h2) {
        add = h2.val + rem;
        rem = 0;
        if(add >= 10) {
            add = add - 10;
            rem = 1
        }
        const newNode = new ListNode(add)
        if(head) {
            head.next = newNode
            head = head.next
        } else {
            head = newNode
            tempHead = head
        }
        h2 = h2.next;
    }
    if(rem) {
        const newNode = new ListNode(rem)
        if(head) {
            head.next = newNode
            head = head.next
        } else {
            head = newNode
            tempHead = head
        }
    }
    return tempHead;
};
