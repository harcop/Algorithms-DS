function stringsConstruction(a: string, b: string): number {
    let _a = a.split("");
    let _b = b.split("");
    _a.sort((a, b) => a.charCodeAt(0) - b.charCodeAt(0));
    _b.sort((a, b) => a.charCodeAt(0) - b.charCodeAt(0)); 
    let _l1 = _a[0];
    let _n1 = 0;
    let _n2 = 0;
    _a.forEach(ele => {
        if (ele === _l1) {
            _n1++;
        }
    })
    _b.forEach(ele => {
        if (ele === _l1) {
            _n2++;
        }
    })
    return _n2/_n1;
}

console.log(stringsConstruction('abc', 'abccba'));