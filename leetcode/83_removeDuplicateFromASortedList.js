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
    const ss = {}
    let newHead = null
    let newCurrent = newHead;
    let current = head
    while(current) {
        let val = current.val
        if(!ss[val]) {
            let newNode = new ListNode(val)
            if(newHead === null) {
                newHead = newNode;
                newCurrent = newNode;
            } else {
                newCurrent.next = newNode
                newCurrent = newCurrent.next
            }
        }
        ss[val] = 1
        current = current.next
    }
    return newHead 
};

function ListNode(val, next) {
    this.val = (val===undefined ? null : val)
    this.next = (next===undefined ? null : next)
}

const head3 = new ListNode(3)
const head22 = new ListNode(2, head3)
const head2 = new ListNode(2, head22)
const head = new ListNode(1, head2)
const head1 = new ListNode(1)

console.log(deleteDuplicates(head1))
console.log(deleteDuplicates(head))
