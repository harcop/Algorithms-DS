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
        let current = this.head;
        if (this.head === null) {
            console.log("am here");
            return false;
        }
        let f = current;
        let s = current;
        console.log("am here", current);
        for (let i = 0; i <= idx; i++) {
            if (current.next === null) {
                console.log("am here");
                // return false;
            }
            current = current.next;
            s = current;
        }
        while (current !== null) {
            s = current.next;
            console.log(current)
            if (current !== null){
                f = f.next;
            }
            current = current.next;
        }
        console.log(f)
        console.log(s)
        let nth = f.next;
        f.next = nth.next;
        return this;
    }
}

const n = new LL();
n.insert(2).insert(4).insert(5);
console.log(n);
console.log(n.remove(2));