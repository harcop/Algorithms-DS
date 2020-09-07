function pagesNumberingWithInk(c: number, n: number): number {
    let falser = true;
    let _n = [];
    while (falser) {
        let _c = c.toString().length;
        if (n >= _c) {
            _n.push(c);
            c++;
            n -= _c;
        }
        else {
            falser = false;
        }
    }
    return _n[_n.length-1];
}

console.log(pagesNumberingWithInk(21, 5));
console.log(pagesNumberingWithInk(8, 4));