const LinkedList = require('./LinkedList');


const ll1 = new LinkedList();
const ll2 = new LinkedList();
ll1.insert(2).insert(6).insert(7).insert(8).insert(11);
ll2.insert(1).insert(3).insert(4).insert(5).insert(9).insert(10);

function ml(l1,l2) {
    let f = l1.head;
    let s = l2.head;
    const l3 = new LinkedList();
    while(f !== null && s !== null) {
        if (f.value < s.value) {
            l3.insert(f.value);
            f = f.next;
        } else {
            l3.insert(s.value);
            s = s.next;
        }
    }
    if (f === null) {
        let current = s;
        while(current !== null) {
            l3.insert(current.value);
            current = current.next;
        }
    }
    else if (s === null) {
        let current = f;
        while(current !== null) {
            l3.insert(current.value);
            current = current.next;
        }
    }
    return l3;
}

console.log(ml(ll2,ll1).traverse());