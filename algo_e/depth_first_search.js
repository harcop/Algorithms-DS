function DBS(name) {
    this.name = name;
    this.children = [];

    this.append = function (name) {
        this.children.push(new DBS(name));
        return this;
    }

    // time O(v+e) and space O(v);
    this.search = function (arr = []) {
        let current = this;
        arr.push(this.name);
        current.children.forEach(child => {
            child.search(arr);
        });
        return arr;
    }
}

const dbs = new DBS('A');
console.log(dbs.append('B'));
console.log(dbs.search());