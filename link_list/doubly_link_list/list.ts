function NoderD(value) {
    this.value = value;
    this.previous = null;
    this.next = null;
}

function DoubleLinkList() {
    this.head = null;
    this.tail = null;

    this.append = function (value) {
        const node = new NoderD(value);
        if(this.head === null) {
            this.head = node;
            this.tail = this.head;
        }
        let current = this.tail;
        current.next = node;
        node.previous = current;
        this.tail = node;
        return this;
    }
}

const dll = new DoubleLinkList();
dll.append(20).append(40).append(30);
console.log(dll);