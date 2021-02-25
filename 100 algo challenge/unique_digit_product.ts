function uniqueDigitProducts(a: number[]): number {
    let _n = [];
    for(let ele of a) {
       let _a = ele.toString().split("");
       let _s = 1;
        _a.forEach(v => {
            _s *= parseInt(v);
        });
        if (!_n.includes(_s)) {
            _n.push(_s);
        }
    }
    return _n.length;
}

console.log(uniqueDigitProducts([2, 8, 121, 42, 222, 23]));