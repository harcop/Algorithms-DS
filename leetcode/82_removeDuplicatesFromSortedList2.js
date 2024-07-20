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
    let tempHead = head;
    let current = head;
    let seen = [];
    let previous = null
    let previous2 = null
    while(current !== null) {
        let currentValue = current.val;
        let i = 0;
        let currentNext = current.next
        if(currentNext) {
            while(currentNext.val === currentValue) {
                
                i++
            }

        }
    }
};

function ListNode(val, next) {
  this.val = (val===undefined ? 0 : val)
  this.next = (next===undefined ? null : next)
}
