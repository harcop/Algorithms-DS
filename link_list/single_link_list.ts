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
        
    }
}

const ll = new LinkList();
ll.append(30).append(20);
console.log(ll);