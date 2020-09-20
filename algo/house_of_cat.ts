function houseOfCats1(l: number): number[] {
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
    let k = 1;
    for(let i = 1; i<=_l[l]; i++) {
        _na.push(k);
        k += 2;
    };
    return _na;
}

function houseOfCats(l: number): number[] {
    let pl: number[] = [];
    while(l >= 0) {
        if (l === 0) {
            break;
        }
        pl.unshift(l/2);
        l -= 4;
    }
    return pl;
}

console.log(houseOfCats(10));
console.log(houseOfCats(6));
