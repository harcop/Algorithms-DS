function NoderT (value) {
    this.value = value;
    this.next = null;
}

function LinkListT() {
    this.head = null;
    this.tail = null;

    this.append = function (value) {
        const node = new NoderT(value);
        let current = this.head;
        if (current === null) {
            this.head = node;
            this.tail = this.head;
        }
        else {
            this.tail.next = node;
            this.tail = node;
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

const llT = new LinkListT();
llT.append(30).append(20).append(40).append(100);
// console.log(llT.remove(20));
// console.log(ll.remove(50));
console.log(llT.traverse());
console.log(llT);