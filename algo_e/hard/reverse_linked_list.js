const LinkedList = require('./LinkedList');

const ll = new LinkedList();
ll.insert(1).insert(2).insert(3).insert(4).insert(5).insert(6).insert(7).insert(8).insert(9);

function rv(l) {
    let p = null;
    let current = l.head;
    while(current !== null) {
        let temp = current.next;
        current.next = p;
        p = current;
        l.head = current
        current = temp;
    }
    return l;
}

console.log(ll);
console.log(ll.traverse());
console.log(rv(ll));
console.log(ll.traverse());