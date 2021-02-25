function chessBoardCellColor(c1: string, c2: string): boolean {
    let _c1 = c1.split("");
    let _c2 = c2.split("");
    let _d1 = _c1[0].charCodeAt(0);
    let _d2 = _c2[0].charCodeAt(0);
    let _e1 = _c1[1].charCodeAt(0);
    let _e2 = _c2[1].charCodeAt(0);
    let _diff1 = Math.abs(_d1 - _d2);
    let _diff2 = Math.abs(_e1- _e2);
    if ((_diff1%2 + _diff2) % 2 === 0) {
        return true;
    }
    return false;

}

console.log(chessBoardCellColor('A1', 'C3'));
console.log(chessBoardCellColor('A1', 'H3'));
console.log(chessBoardCellColor('A1', 'B4'));
