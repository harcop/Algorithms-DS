function mostFrequentDigitSum1(n) {
    let _n = [];
    while (n > 0) {
        let _d = n.toString().split("");
        let _s = _d.map(a => parseInt(a));
        let s = _s.reduce((a ,b) => a + b);
        _n.push(s);
        n = n - s;
    }
    _n.push(0);
    let _d = _n.map(a => {
        let _s = a.toString().split("").map(b => parseInt(b));
        let s = _s.reduce((c ,d) => c + d);
        return s;
    });
    let c = {};
    _d.forEach(ele => {
        if (!c[ele]) {
            c[ele] = 0;
        }
        c[ele] += 1;
    });
    let _m = 0;
    for(let i in c) {
        if (c[parseInt(i)] >= c[_m]) {
            _m = parseInt(i);
        }
    }
    return _m;
 
}

function mostFrequentDigitSum (n) {
    if (n > 9) {
        return 9
    }
    return n;
}

console.log(mostFrequentDigitSum(88));
console.log(mostFrequentDigitSum(8));