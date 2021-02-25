function matrixElementsSum(m: any[][]): number {
    let _fp = 0;
    let _h = [];
    for(let y=0; y<m.length; y++) {
        for(let x =0; x < m[y].length; x++) {
            let ele = m[y][x];
            if (ele === 0) {
                _h.push(x);
                continue;
            }
            if (y>0) {
                if (_h.includes(x)) {
                    continue;
                }
            }
            console.log(y, ele)
            _fp += ele;
        }
    }
    console.log(_fp);
    return _fp;
}   

console.log(matrixElementsSum(
    [[0, 1, 1, 2],
    [1, 5, 0, 0],
    [2, 0, 3, 3]]
));
