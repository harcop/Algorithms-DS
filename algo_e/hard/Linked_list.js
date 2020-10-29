function Node (value) {
    this.value = value;
    this.next = null;
    this.isVisited = false;
}

function LinkedList () {
    this.head = null;

    this.insert = (value, p = null) => {
        let node = new Node(value);
        let current = this.head;
        let i = 0;
        if (current === null) {
            this.head = node;
        }else {
            while(current.next !== null) {
                if (p && i===p) {
                    p = current;
                }
                current = current.next;
                i++;
            }
            current.next = node;
            node.next = p;
        }
        return this;
    }

    this.traverse = () => {
        let current = this.head;
        let _f = [];
        while (current !== null) {
            _f.push(current.value);
            current = current.next;
        }
        console.log(_f);
        return _f;
    }
}

const ll = new LinkedList();
ll.insert(1).insert(2).insert(3).insert(4).insert(5).insert(6).insert(7).insert(8).insert(9, 3);
console.log(ll);

module.exports = LinkedList;