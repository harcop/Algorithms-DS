function companyBotStrategy(td: number[][]): number {
    let _t = [];
    td.forEach(ele => {
        if (ele[1] > 0) {
            _t.push(ele[0]);
        }
    })
    let _r = _t.reduce((a,b) => a+b, 0)/_t.length;
    if (_r > 0) {
        return _r
    }
    return 0.0;
}

console.log(companyBotStrategy([[3, 1], [6, 1], [4, 1], [5, 1]]));
console.log(companyBotStrategy([[4, 1], [4, -1], [0, 0], [6, 1]]));
console.log(companyBotStrategy( [[4, -1], [0, 0], [5, -1]]));
