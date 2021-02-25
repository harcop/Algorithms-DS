const LinkedList = require('./LinkedList');

const ll = new LinkedList();
ll.insert(1).insert(2).insert(3).insert(4).insert(5).insert(6).insert(7).insert(8).insert(9, 3);
console.log(ll);

// function fl1 (l) {
//     let current = l.head;
//     // console.log(current)
//     while(current.next !== null && !current.next.isVisited) {
//         current.isVisited = true;
//         current = current.next;
//     }
//     if (current.next === null) {
//         return false;
//     }
//     return true;
// }

function fl(l) {
    let f = l.head.next;
    let s = l .head.next.next;
    
    while(f.next !== s.next) {
        f = f.next;
        s = s.next.next;
    }
    f = l.head;
    while(f.next !== s.next) {
        f = f.next;
        s = s.next;
    }
    return f.next.value;
}
console.log(fl(ll));