function Node(value) {
    this.value = value;
    this.next = null;
}

function LL () {
    this.head = null;

    this.insert = (value) => {
        let node = new Node(value);
        let current = this.head;
        if (current === null) {
            this.head = node;
        } else {
            while(current !== null) {
                if (current.next === null) {
                    current.next = node;
                    break;
                }
                current = current.next;
            }
        }
        return this;
    }

    this.remove = (idx) => {
        let head = this.head;
        if (head === null || head.next === null) {
            return null;
        }
        let f = head;
        let s = head;
        for (let i = 0; i < idx; i++) {
            s = s.next;
        }
        if (s === null) {
            head.value = head.next.value;
            head.next = head.next.next;
            return this;
        }
        while (s.next !== null) {
            s = s.next;
            f = f.next;
        }
        f.next = f.next.next;
        return this;
    }
}

const n = new LL();
n.insert(2).insert(4).insert(5).insert(6);
console.log(n);
console.log(n.remove(3));