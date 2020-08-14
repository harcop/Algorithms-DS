function NodeC(value) {
    this.value = value;
    this.next = null;
    this.previous = null;
}

function Circular() {
    this.head = null;
    this.tail = null;

    this.append = function (value) {
        const node = new NodeC(value);
        let current = this.head;
        if (current === null) {
            this.head = node;
            this.tail = this.head;
            this.head.previous = this.tail;
            this.head.next = this.tail;
        }
        else {
            current = this.tail;
            current.next = node;
            node.previous = current;
            node.next = this.head;
            this.head.previous = node;
            this.tail = node;
        }
        return this;
    }

    this.traverse = function () {
        let current = this.head;
        let nodes = [];
        while(current.next !== this.head) {
            nodes.push(current.value);
            current = current.next;
        }
        nodes.push(current.value);
        return nodes;
    }
}

const llC = new Circular();
llC.append(30).append(60).append(90).append(890);
console.log(llC.traverse());