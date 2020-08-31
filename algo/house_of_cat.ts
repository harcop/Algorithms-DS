function houseOfCats(l: number): number[] {
    let _a = [2,4];
    let _l = Array(l+1);
    _l.fill(0,0,l+1);
    _l[0] = 1;
    _a.forEach(ele => {
        if (ele > l) return;
        for(let i=ele; i<_l.length; i = i+2) {
            let _d = i - ele;
            _l[i] += _l[_d];
        }
    });
    let _na = [];
    let k = l%4===0 ? 2 : 1;
    for(let i = 0; i<_l[l]; i++) {
        _na.push(k);
        k += 2;
    };
    l%4===0 ? _na.pop(): _na;
    return _na;
}

console.log(houseOfCats(10));
console.log(houseOfCats(6));
console.log(houseOfCats(2));
console.log(houseOfCats(20));
console.log(houseOfCats(12));
console.log(houseOfCats(8));
