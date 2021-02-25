function Node (value) {
    this.name = value;
    this.children = []

    this.insert = (value) => {
        const node = new Node(value);
        this.children.push(node);
        console.log(this.children[0])
    }

    //time O(v+e) //space O(v) //v = vertices/node && e=edges
    this.bfs = (array = []) => {
        queue = [this];
        while(queue.length > 0) {
            let current = queue.splice(0,1)[0];
            array.push(current.name);
            for (const c of current.children) {
               queue.push(c);
           }
        }
        return array;
    }
}

const node = new Node("A");
node.insert("B");
console.log(node.bfs());