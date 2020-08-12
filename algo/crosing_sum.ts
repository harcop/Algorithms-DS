function crossingSum(m: number[][], a: number, b: number): number {
    let _a = m[a];
    let _sumA = _a.reduce((a,b) => a+b, 0);
    m.forEach((ele, index) => {
        if (index === a) {
            return;
        }
        _sumA += ele[b];
    }) 
    return _sumA;
}

console.log(crossingSum([[1, 1, 1, 1], 
    [2, 2, 2, 2], 
    [3, 3, 3, 3]], 1, 3));