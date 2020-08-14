function Noder (value) {
    this.value = value;
    this.next = null;
}

function LinkList() {
    this.head = null;

    this.append = function (value) {
        const node = new Noder(value);
        let current = this.head;
        if (current === null) {
            this.head = node;
        }
        else {
            while(current.next !== null) {
                current = current.next;
            }
            current.next = node;
        }
        return this;
    }

    this.traverse = function () {
        let current = this.head;
        let nodes = [];
        while(current !== null) {
            nodes.push(current.value);
            current = current.next;
        }
        return nodes;
    }

    this.remove = function (value) {
        let current = this.head;
        let falser = true;
        while(current.next !== null) {
            if (current.next.value === value) {
                current.next = current.next.next;
                return true;
            }
            current = current.next;
        }
        return false
    }
}

const ll = new LinkList();
ll.append(30).append(20).append(40);
console.log(ll.remove(20));
console.log(ll.remove(50));
console.log(ll.traverse());